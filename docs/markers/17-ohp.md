# 17-Hydroxyprogesterone (`17-ohp`)

Steroid intermediate of cortisol synthesis, raised by 21-hydroxylase deficiency. Used in newborn screening for congenital adrenal hyperplasia and, with an ACTH-stimulated test, to diagnose non-classic CAH.

| | |
|---|---|
| Category | [adrenal](index.md#adrenal) |
| Canonical unit | ng/dL |
| LOINC | - |
| Specimen | - |
| Aliases | `17-oh-progesterone` |

## Ranges

Bounds in ng/dL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | male, fasting | 28.75 | 206.2 | reference-interval-study | 525 volunteers aged 18-81 (232 men), with and without hypertension; morning fasting supine sampling (specimen: plasma; method: lc-ms/ms) | [Reference intervals for plasma concentrations of adrenal steroids measured by LC-MS/MS: Impact of gender, age, oral contraceptives, body mass index and blood pressure status.](https://pubmed.ncbi.nlm.nih.gov/28479316/), Table 3 (columns: ALL \| Males \| Females; median then [2.5th-97.5th percentile]) | 2.5th-97.5th percentile for men (median 2.46 nmol/L). Converted from the source's nmol/L (0.87-6.24) to ng/dL using factor 33.05. Applies only to: time_of_day=morning. |
| adrenal-steroids-lcms-2017 | reference | female, fasting | 7.932 | 226.1 | reference-interval-study | 525 volunteers aged 18-81 (293 women, any cycle phase, menopausal status or oral contraceptive use), with and without hypertension (specimen: plasma; method: lc-ms/ms) | [Reference intervals for plasma concentrations of adrenal steroids measured by LC-MS/MS: Impact of gender, age, oral contraceptives, body mass index and blood pressure status.](https://pubmed.ncbi.nlm.nih.gov/28479316/), Table 3 (columns: ALL \| Males \| Females; median then [2.5th-97.5th percentile]) | 2.5th-97.5th percentile for women pooled across cycle phases (median 0.79 nmol/L); luteal-phase values run higher. Converted from the source's nmol/L (0.24-6.84) to ng/dL using factor 33.05. Applies only to: time_of_day=morning. |
| cah-practice-guideline-restated | warn | all | - | 991.5 | consensus | Patients evaluated for nonclassic CAH (basal 17OHP before ACTH stimulation), as restated from CAH practice guidelines | [Screening for Nonclassic Congenital Adrenal Hyperplasia in the Era of Liquid Chromatography-Tandem Mass Spectrometry.](https://pubmed.ncbi.nlm.nih.gov/32110745/), Introduction (full text) | Borderline zone for basal 17OHP: 6 to 30 nmol/L prompts a 250 mcg ACTH stimulation test. Restates CAH guideline values; the sentence does not name the guideline (the paywalled Endocrine Society 2018 CAH guideline is the presumed source, not verified here). Values below 6 nmol/L are said to exclude CAH. Converted from the source's nmol/L (6-30) to ng/dL using factor 33.05. Source band 198.3-991.5; stored as the warn high bound 991.5. |
| cah-practice-guideline-restated | warn | all | - | 991.5 | consensus | Patients evaluated for nonclassic CAH after a 250 mcg ACTH stimulation test, as restated from CAH practice guidelines (test: cosyntropin 250 ug stimulation, 60 min) | [Screening for Nonclassic Congenital Adrenal Hyperplasia in the Era of Liquid Chromatography-Tandem Mass Spectrometry.](https://pubmed.ncbi.nlm.nih.gov/32110745/), Introduction (full text) | Post-stimulation 17OHP at 60 minutes: values above 30 nmol/L are diagnostic of nonclassic CAH. Restates guideline values (guideline not named in the sentence). Nonclassic CAH values in ng/dL are not restated here. Converted from the source's nmol/L (30) to ng/dL using factor 33.05. Source kind: diagnostic. Source band >= 991.5; stored as the warn high bound 991.5. |
| lcms-17ohp-ncah-2020 | warn | all | - | 198.3 | cohort | Retrospective single-centre chart review (n=188) of patients tested for 17OHP; LC-MS/MS measurement, basal (baseline) 17OHP (method: lc-ms/ms) | [Screening for Nonclassic Congenital Adrenal Hyperplasia in the Era of Liquid Chromatography-Tandem Mass Spectrometry.](https://pubmed.ncbi.nlm.nih.gov/32110745/), Abstract, Conclusions | Baseline (basal) 17OHP screening cut-off in nmol/L by LC-MS/MS; the authors report it as the most accurate screening value for nonclassic CAH and state it supports current guidelines. Values at or above it warrant ACTH stimulation testing. Not an ng/dL value (ng/dL not stated in source). Converted from the source's nmol/L (6) to ng/dL using factor 33.05. Source kind: diagnostic. Source band >= 198.3; stored as the warn high bound 198.3. |
| newborn-screening-2026 | warn | newborn-screening | - | 1200 | cohort | Neonates in a CAH newborn-screening programme (tertiary referral centre); the authors' referral cut-off that triggered neonatal recall | [Early Risk Stratification in Non-Classical Congenital Adrenal Hyperplasia Based on Newborn 17-OHP Screening Values, Hormonal Findings, and Genotype.](https://pubmed.ncbi.nlm.nih.gov/41976932/), Results | Newborn screening recall cut-off in ng/mL as used by the authors' programme, not a guideline threshold. Values below it can still represent non-classic CAH (the source reports mildly elevated values below recall in most cases). Basal and ACTH-stimulated cut-offs from the 2018 Endocrine Society CAH guideline could not be verified with the available tools. Converted from the source's ng/mL (12) to ng/dL using factor 100. Source kind: diagnostic. Source band >= 1200.0; stored as the warn high bound 1200.0. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
