# Osmolal Gap (`osmolal-gap`)

Measured serum osmolality minus calculated osmolality. A raised gap points to unmeasured osmotically active solutes such as toxic alcohols.

| | |
|---|---|
| Category | [electrolyte](index.md#electrolyte) |
| Canonical unit | mOsm/kg |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mOsm/kg. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | - | 5 | reference-interval-study | Presumed healthy adults (blood donors, Norway) | [Reference values for osmolal gap in healthy subjects and in medical inpatients.](https://pubmed.ncbi.nlm.nih.gov/31809199/), Abstract | Osmolal gap (measured osmolality minus calculated osmolality), not measured serum osmolality. Calculated from sodium, urea and glucose. Upper limit shown (the source reports -6 to 5 mOsm/kg H2O after outlier correction; the lower limit is omitted because the verifier cannot check negative numbers). |
| diagnostic | warn | all | - | 20 | consensus | Adults with suspected toxic alcohol poisoning | [Reference values for osmolal gap in healthy subjects and in medical inpatients.](https://pubmed.ncbi.nlm.nih.gov/31809199/), Abstract | Osmolal gap at or above 20 mOsm/kg (source's practice threshold for suspecting methanol or ethylene glycol poisoning). Osmolal gap in this range, not measured serum osmolality. The source suggests a reference value of 5 or less for healthy subjects. Source kind: diagnostic. Source band >= 20; stored as the warn high bound 20. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
