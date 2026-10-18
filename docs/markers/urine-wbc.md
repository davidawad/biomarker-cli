# Urine White Blood Cells (microscopy) (`urine-wbc`)

White blood cells counted on microscopy of a urine sediment, per high-power field. Pyuria (raised WBC) suggests urinary tract infection or inflammation, but its predictive value depends on how concentrated the urine is.

| | |
|---|---|
| Category | [urinalysis](index.md#urinalysis) |
| Canonical unit | /HPF |
| LOINC | [5821-4](https://loinc.org/5821-4/) |
| Specimen | - |
| Aliases | `pyuria`, `urine leukocytes`, `urine wbc` |

## Ranges

Bounds in /HPF. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 5 | 10 | consensus | Adults: commonly accepted pyuria cut-off range, as cited in a study of older women | [Current Pyuria Cutoffs Promote Inappropriate Urinary Tract Infection Diagnosis in Older Women.](https://pubmed.ncbi.nlm.nih.gov/36806580/), Introduction (full text) | Commonly accepted pyuria cut-off of 5 to 10 leukocytes per high-power field (adult background cited in the 2023 Clinical Infectious Diseases study). The study itself argues these cut-offs are too low for older women. Lower bound 5 is the threshold; the upper bound is the cited upper end of the band. Source kind: decision. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| alt-1 | warn | all | - | 6 | cohort | Children under 24 months evaluated in the emergency department for suspected UTI, with paired urinalysis and urine culture | [Pyuria and Urine Concentration for Identifying Urinary Tract Infection in Young Children.](https://pubmed.ncbi.nlm.nih.gov/33514634/), Abstract, Results; [Pyuria and Urine Concentration for Identifying Urinary Tract Infection in Young Children.](https://pubmed.ncbi.nlm.nih.gov/33514634/), Abstract, Methods | Optimal microscopic WBC cut-off per HPF at moderate urine concentration (USG 1.011 to 1.020), against a positive urine culture. Retrospective study in young children, not adults. Applies only to: markers={"urine-specific-gravity": "1.011-1.020"}. Source kind: decision. Source band >= 6; stored as the warn high bound 6. Alternative to the default pediatric-pyuria-sg-2021 warn range for the same population. |
| alt-2 | warn | all | - | 8 | cohort | Children under 24 months evaluated in the emergency department for suspected UTI, with paired urinalysis and urine culture | [Pyuria and Urine Concentration for Identifying Urinary Tract Infection in Young Children.](https://pubmed.ncbi.nlm.nih.gov/33514634/), Abstract, Results; [Pyuria and Urine Concentration for Identifying Urinary Tract Infection in Young Children.](https://pubmed.ncbi.nlm.nih.gov/33514634/), Abstract, Methods | Optimal microscopic WBC cut-off per HPF at high urine concentration (USG >1.020), against a positive urine culture. Retrospective study in young children, not adults. Applies only to: markers={"urine-specific-gravity": ">1.020"}. Source kind: decision. Source band >= 8; stored as the warn high bound 8. Alternative to the default pediatric-pyuria-sg-2021 warn range for the same population. |
| pediatric-pyuria-sg-2021 | warn | all | - | 3 | cohort | Children under 24 months evaluated in the emergency department for suspected UTI, with paired urinalysis and urine culture | [Pyuria and Urine Concentration for Identifying Urinary Tract Infection in Young Children.](https://pubmed.ncbi.nlm.nih.gov/33514634/), Abstract, Results; [Pyuria and Urine Concentration for Identifying Urinary Tract Infection in Young Children.](https://pubmed.ncbi.nlm.nih.gov/33514634/), Abstract, Methods | Optimal microscopic WBC cut-off per HPF at low urine concentration (USG <1.011), against a positive urine culture (AAP culture thresholds). Cut-off applies to low-concentration urine only. Retrospective study in young children, not adults. Applies only to: markers={"urine-specific-gravity": "<1.011"}. Source kind: decision. Source band >= 3; stored as the warn high bound 3. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
