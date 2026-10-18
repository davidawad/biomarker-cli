# Anion Gap (`anion-gap`)

The anion gap is calculated from serum sodium, chloride and bicarbonate (and sometimes potassium) and is used to classify metabolic acidoses. Its reference interval depends on the formula and assay used, so the laboratory's own interval should be applied.

| | |
|---|---|
| Category | [electrolyte](index.md#electrolyte) |
| Canonical unit | mmol/L |
| LOINC | [33037-3](https://loinc.org/33037-3/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mmol/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 7 | 18 | reference-interval-study | Healthy adults (Yale-New Haven Hospital reference population; no significant sex or age differences) | [Interpreting Anion Gap Values in Adult and Pediatric Patients: Examining the Reference Interval.](https://pubmed.ncbi.nlm.nih.gov/32445342/), Abstract, Results | Derived from 17,137 patients (healthy adults, ED and ICU). The abstract does not state the formula or method (with or without potassium), so no method condition is set. The previous laboratory interval was 6-16 mmol/L. |
| alt-1 | reference | all | 10 | 16 | reference-interval-study | Healthy medical students (aged 20-32) and health professionals (aged 21-76), arterial blood gas analyser samples (Radiometer ABL) (method: with-potassium; specimen: arterial) | [Reference intervals and age and gender dependency for arterial blood gases and electrolytes in adults.](https://pubmed.ncbi.nlm.nih.gov/21619466/), Abstract | Anion gap with potassium included in the calculation (formula not stated in the abstract). Arterial blood gas samples; not a serum interval. Alternative to the default clinical reference range for the same population. |
| alt-2 | reference | all | 6 | 12 | reference-interval-study | Healthy medical students (aged 20-32) and health professionals (aged 21-76), arterial blood gas analyser samples (Radiometer ABL) (method: without-potassium; specimen: arterial) | [Reference intervals and age and gender dependency for arterial blood gases and electrolytes in adults.](https://pubmed.ncbi.nlm.nih.gov/21619466/), Abstract | Anion gap calculated without potassium (formula not stated in the abstract). Arterial blood gas samples; not a serum interval. Alternative to the default clinical reference range for the same population. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
