# Beta-Hydroxybutyrate (Ketones, Blood) (`beta-hydroxybutyrate`)

Blood beta-hydroxybutyrate is the main ketone body and the preferred measure of ketonaemia. Measured directly (capillary or venous) it is more specific for diabetic ketoacidosis (DKA) than urine or acetoacetate tests.

| | |
|---|---|
| Category | [chemistry](index.md#chemistry) |
| Canonical unit | mmol/L |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in mmol/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | diabetes | 0.6 | 1.5 | consensus | People with diabetes on sick-day rules (euglycaemic DKA prevention, medication stopped during illness) | [Developing a Protocol for Management of Euglycemic Diabetic Ketoacidosis.](https://pubmed.ncbi.nlm.nih.gov/40900206/), Management, sick day rules paragraph | Trace/small ketones or blood ketones 0.6-1.5 mmol/L during illness: take carbohydrate, correction insulin, fluids and recheck ketones every 3-4 hours, per the sick-day advice in the source review. This is a home-monitoring action level, not a diagnosis. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| diagnostic | warn | diabetes | - | 3 | consensus | Adults with diabetes presenting with suspected hyperglycaemic crisis | [Hyperglycaemic crises in adults with diabetes: a consensus report.](https://pubmed.ncbi.nlm.nih.gov/38907161/), Full text, diagnosis of DKA | Blood beta-hydroxybutyrate of 3.0 mmol/L or higher supports a diagnosis of DKA (with hyperglycaemia and acidosis), per the 2024 ADA/EASD/JBDS/AACE/DTS consensus. Source kind: diagnostic. Source band >= 3.0; stored as the warn high bound 3.0. |
| target | optimal | ketogenic-diet | 0.5 | - | cohort | Adults following a ketogenic diet (14-day daily beta-hydroxybutyrate monitoring, 217 people) | [Defining a ketone threshold for weight loss: evidence from 14 day daily β-hydroxybutyrate monitoring in 217 subjects on a ketogenic diet.](https://pubmed.ncbi.nlm.nih.gov/42449121/), Abstract (Conclusions) | Nutritional ketosis threshold associated with weight loss in one cohort. Not a diagnostic or clinical safety range; the source gives no upper limit. Source kind: target. |
| beta-hydroxybutyrate-bands-1-6-3-0 | reference | diabetes | 1.6 | 3 | consensus | People with diabetes on sick-day rules (euglycaemic DKA prevention) | [Developing a Protocol for Management of Euglycemic Diabetic Ketoacidosis.](https://pubmed.ncbi.nlm.nih.gov/40900206/), Management, sick day rules paragraph | Moderate ketones: the source says the same advice can be followed, but medical attention is recommended. Above 3.0 mmol/L the source advises immediate action (wording is cut off in the retrieved text); the DKA threshold of 3.0 mmol/L is in the diagnostic range above. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
