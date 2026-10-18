# Absolute eosinophil count (`eosinophils-abs`)

Absolute count of circulating eosinophils, involved in allergy, asthma and parasitic infection. High values (eosinophilia) suggest allergy, drug reaction (DRESS), parasitosis, or hypereosinophilic / myeloid disorders; a count of 1.5 x 10^9/L or more defines eosinophilia for hypereosinophilic syndrome work-up.

| | |
|---|---|
| Category | [cbc](index.md#cbc) |
| Canonical unit | 10^3/µL |
| LOINC | [711-2](https://loinc.org/711-2/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in 10^3/µL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 0 | 0.7 | reference-interval-study | Black volunteers from pooled screening/admission data of 35 UK phase I trials, morning sampling | [Time- and Race-Specific Haematological Reference Intervals for Healthy Volunteer Trials: A Retrospective Analysis of Pooled Data From Multiple Phase I Trials.](https://pubmed.ncbi.nlm.nih.gov/32231575/), Table 5, proposed reference intervals | Race- and time-specific adjusted interval; evening interval is 0.0-0.7 (not separately encoded). Eosinophil counts fall over the day in non-Black volunteers. Applies only to: ethnicity=black. |
| alt-1 | reference | all | 0 | 0.6 | reference-interval-study | Non-Black volunteers from pooled screening/admission data of 35 UK phase I trials, morning sampling | [Time- and Race-Specific Haematological Reference Intervals for Healthy Volunteer Trials: A Retrospective Analysis of Pooled Data From Multiple Phase I Trials.](https://pubmed.ncbi.nlm.nih.gov/32231575/), Table 5, proposed reference intervals | Race- and time-specific adjusted interval; evening interval is 0.0-0.5 (not separately encoded). Applies only to: ethnicity=nonblack. Alternative to the default time-race-specific-ri reference range for the same population. |
| barcelona-sysmex-xn | reference | all | 0.03 | 0.39 | reference-interval-study | 181 apparently healthy adults from the southern metropolitan area of Barcelona, Sysmex XN | [Reference intervals for a complete blood count on an automated haematology analyser Sysmex XN in healthy adults from the southern metropolitan area of Barcelona.](https://pubmed.ncbi.nlm.nih.gov/29765286/), Results, CBC reference intervals table | Analyser-specific interval (Sysmex XN). |
| hes-definition-review | warn | all | - | 1.5 | consensus | Patients with peripheral blood eosinophilia evaluated for hypereosinophilic syndrome (HES); review of the diagnostic definition | [How I treat hypereosinophilic syndromes.](https://pubmed.ncbi.nlm.nih.gov/25964669/), Abstract | Diagnostic threshold for eosinophilia used in HES definition (count >= 1.5 plus end-organ involvement). The 0.5 x 10^9/L eosinophilia cutoff was not found in retrieved text and is not encoded. Source kind: diagnostic. Source band >= 1.5; stored as the warn high bound 1.5. |
| kenya-urban-adults | reference | all | 0.04 | 0.59 | reference-interval-study | healthy adult urban population in Kenya (Nairobi), Beckman Coulter ACT 5 DIFF CP (region: Kenya) | [Complete blood count reference intervals from a healthy adult urban population in Kenya.](https://pubmed.ncbi.nlm.nih.gov/29879171/), Table, reference intervals | All-participant interval (n = 460) from the reference-interval table; the discussion also cites a parametrically derived upper limit of 590 cells/µL. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
