# FHIR R4 and HL7 v2

`biomarker import` reads FHIR R4 JSON and HL7 v2 ORU^R01 messages; `biomarker
export --format fhir` writes FHIR R4. Nothing is sent anywhere: files in, files
out. There is no network code for health data.

## Where the files come from

Patient portals and apps that offer "download my record" give FHIR JSON
(single resources, Bundles, or NDJSON). Test data generators such as Synthea
write the same shapes. Lab-to-EHR interface messages are HL7 v2; individuals
rarely receive them, but hobbyists and researchers running their own interface
do. The importer reads whatever it recognises and reports what it set aside,
so a portal export that mixes lab results with medications and encounters
works: the other resource types are counted and ignored.

## Import

```console
$ biomarker import record.json --person alex               # a Bundle or one resource
$ biomarker import observations.ndjson --person alex       # one resource per line
$ biomarker import ~/Downloads/fhir/ --person alex         # every *.json / *.ndjson in a directory
$ biomarker import results.hl7 --person alex               # HL7 v2 (also a directory of *.hl7)
$ biomarker import record.json --person alex --dry-run     # report only; nothing written
```

Detection is by `--input-format fhir|fhir-ndjson|hl7v2`, else by extension
(`.hl7`, `.oru`, `.fhir`, `.ndjson`), else by looking at the start of the file
(`MSH|` or a `"resourceType"`). Reading from standard input (`-`) needs the
explicit `--input-format`.

A result is read from an `Observation` (laboratory and vital-signs
categories by default; `--categories laboratory,vital-signs,...` narrows or
widens). `DiagnosticReport` and `Specimen` supply the lab name, the collection
time and fasting status through the references that connect them. A blood
pressure panel becomes one result per component (systolic, diastolic); panel
headers that only group other results are skipped.

### Which person

`--person` (or `default_person`) names the person. With neither, the only
person in the database is used; with several, the import stops rather than
guess. The file's `Patient` is used for a sanity check only: if its gender or
birth date disagrees with the person, the import stops unless `--force-person`
is given. Names, addresses, identifiers and telecom are never stored.
`--create-people` creates the person, taking sex and birth date from a lone
Patient.

### Which marker

1. A LOINC code in the mapping file's `[loinc]` table (`--mapping FILE`).
2. A LOINC code known to the database: the catalog's code for each marker, plus
   any added with `biomarker marker loinc add MARKER CODE [--relation
   exact|variant]`. A marker can have several codes (a calculated and a direct
   variant of one analyte). `marker loinc list [MARKER]` shows them;
   `marker loinc remove MARKER CODE` removes one you added.
3. The test's name (code text and codings' display) through the marker names
   and aliases, like a spreadsheet import. The mapping file's `[markers]`
   table renames source names.

What nothing maps to is handled by `--unmapped`:

* `report` (default): counted, listed in the summary and kept in the
  `fhir_unmapped` table with the reason, so nothing is lost silently.
* `skip`: counted and dropped.
* `create`: a custom marker is created (category from the FHIR category, unit
  from the first result, LOINC recorded). Catalog markers are never touched.

### Units

A numeric result carries a UCUM code. The importer maps UCUM to the repo's
unit spellings with a table (`src/interop/ucum.rs`: `mg/dL`, `ug/dL` ->
`µg/dL`, `10*3/uL` -> `10^3/µL`, `[IU]/L` -> `IU/L`, `m[IU]/L` -> `mIU/L`,
`mL/min/{1.73_m2}` -> `mL/min/1.73m²`, `1` -> `ratio`, ...), strips `{...}`
annotations, and converts to the marker's canonical unit with the same
conversion table the rest of the CLI uses. `value_raw` and `unit_raw` keep
what the source reported. A result with no unit, or a unit with no conversion
to the marker's, is never guessed: it is counted as `unconvertible`, kept in
`fhir_unmapped`, and not imported. `Quantity.comparator` (`<`, `<=`, `>=`,
`>`) becomes the measurement's `qualifier`.

### Time

`taken_at` is the Observation's `effective[x]`, else the Specimen's
collection time, else the DiagnosticReport's `effective[x]`, else `issued`
(HL7 v2: OBX-14, else OBR-7, else the specimen's collection time). A time with
an offset is converted to the configured zone; a date stays a date. The source
used is recorded with the result.

### The lab's reference range and flag

A lab range belongs to one reported result (lab, method, age, sex, date), so
it is **never** written to `ranges`, `person_ranges` or the catalog's cited
population ranges. It is stored per result (`lab_result_meta`): every range as
reported, the range that fits the person converted to the canonical unit, the
lab's text, and its interpretation codes (`N L H LL HH A AA < >`).

`query`, `latest` and `flag` in JSON gain `lab_ref_low`, `lab_ref_high`,
`lab_ref_text`, `lab_interpretation` and `lab_flag_agrees` (is the lab's
flag the same as the catalog's) whenever the database holds any imported
result. Databases without one print exactly what they did before.

### Re-importing, amendments, retractions

Every imported result remembers where it came from: the Observation's
`identifier` (`system|value`), else its place in the file, plus a hash of its
content (the server-assigned `meta` is ignored). HL7 v2 results are keyed by
sending facility, filler order number, OBX set id and observation id.

| situation | outcome |
|-----------|---------|
| same source, same content | `skipped` (nothing written) |
| same source, changed content (corrected, amended) | `replaced`: the same row is updated |
| new source, but a measurement for that person, marker and instant exists | the `--dedupe` policy (`skip` default, `replace`, `error`); with `--link-existing`, a skipped result is still linked to its source so later amendments apply |
| new source, no such measurement | `inserted` |
| a different source reports the same marker at the same instant | `conflict`: the first result wins and the second is listed in the summary (the natural key cannot hold both) |
| status `entered-in-error`, `cancelled` (HL7 v2: `D`, `W`) | counted as `retracted` when it was imported before; deleted only with `--apply-retractions` |
| status `preliminary` or `registered` (HL7 v2: `P`, `R`, `I`, `S`, `O`) | skipped unless `--include-preliminary`, then tagged `preliminary` |

Importing the same file twice reports 0 inserted and 0 replaced. An import
runs in one transaction; `--dry-run` rolls it back.

Qualitative results (`valueString`, `valueCodeableConcept`, `valueBoolean`)
go to `observations`; an `A`/`AA` interpretation sets the flag `abnormal`.
Values of other types (Ratio, Range, SampledData, attachments) are counted
as `unsupported_value`.

### The summary

`biomarker import ... -f json` prints one object in the `data` field:
`batch`, `source`, `format`, `dry_run`, `person`, `dedupe`, and the counts
`results`, `inserted`, `replaced`, `skipped`, `retracted`,
`retractions_applied`, `conflicts`, `unmapped`, `unconvertible`, `invalid`;
`observations` (`{inserted, replaced, skipped}`); `ignored` and
`ignored_resources` (set-aside resource types and their total);
`skipped_category` and `skipped_status` (counts by reason); `panel_headers`,
`no_value`, `unsupported_value`; `markers_created`; the lists
`unmapped_results`, `unconvertible_results`, `conflict_results`,
`skipped_names`, `warnings` and `errors`. Fields are only ever added.

## Export

```console
$ biomarker export --person alex --format fhir -o labs.fhir.json
$ biomarker export --person alex --format fhir-ndjson
```

`--format fhir` writes a `Bundle` of `type: collection` holding a `Patient`
(gender and birth date; the name only with `--include-name`), one
`Observation` per measurement shaped after the US Core Laboratory Result
profile, and a `Specimen` where the lab, fasting status or specimen type is
known. `--format fhir-ndjson` writes the same resources one per line. The
FHIR formats print FHIR, not the `biomarker/v1` envelope, and exist for
`export` only. `--person` is required (one Patient per bundle).

Per measurement:

* `code`: the marker's LOINC code (with the marker name as display and text).
  A marker with no LOINC code is written with `code.text` only, tagged
  `no-loinc` in `meta.tag` and the Bundle's `meta.tag`, and listed in a
  warning on standard error.
* `effectiveDateTime`: a date stays a date; a time carries the configured
  zone's offset (FHIR requires one).
* `valueQuantity`: the value and unit **as reported** (`value_raw`,
  `unit_raw`) with the UCUM `system`/`code`. A unit with no UCUM equivalent
  is written as `unit` text only, and listed in a warning. With an explicit
  `--units` (`canonical`, `si`, `us`), values are written in that
  system's units instead. `qualifier` becomes `comparator`.
* `referenceRange`: the lab's own ranges as imported, by default
  (`--ranges lab`). `--ranges catalog` adds the catalog's cited range as a
  second entry with `type.text = "biomarker-cli catalog"` and a `text` that
  says it is not the lab's. `--ranges none` omits ranges.
* `interpretation`: the lab's codes if stored. `--interpret catalog` adds one
  computed from the catalog range (`H`, `L`, `N`), marked
  `text = "biomarker-cli catalog"`.
* `status`: `final`, or `amended`/`corrected` for a result imported as such.
* `identifier`: system `urn:biomarker-cli:<database uuid>`, value the
  measurement id. Importing an export is idempotent: into the same database
  it adds nothing, into another it inserts once and then skips.
* `--group-reports` wraps results sharing lab, date and import batch in a
  `DiagnosticReport`.

Qualitative results export as `valueString` (or `valueCodeableConcept` for
words such as Positive, Negative, Detected) with an `A` interpretation when
flagged abnormal.

## Limits

* One person per file. Cross-file references are resolved only when the files
  are imported together (a directory).
* The importer reads laboratory and vital-signs Observations. Older FHIR
  versions are read tolerantly (a single object where R4 has an array), but
  only R4 is tested.
* Two different results for the same marker at the same instant cannot both
  be stored (the natural key is unchanged): the first wins and the rest are
  reported as `conflicts`.
* Reading directly from a provider's FHIR API (SMART on FHIR) is not part of
  this release.
* The HL7 v2 reader follows the v2-to-FHIR field mapping for ORU^R01. Result
  status codes it does not recognise (anything other than the ones listed
  above) are skipped rather than guessed.
