# Tryptase (`tryptase`)

Mast cell protease measured in serum. Basal (resting) tryptase screens for mastocytosis, hereditary alpha-tryptasemia (HaT) and other causes of elevation; acute tryptase during suspected anaphylaxis is compared with baseline. Expected result in healthy people: basal tryptase within the normal range of 1 to 15 ng/mL.

| | |
|---|---|
| Category | [allergy](index.md#allergy) |
| Canonical unit | ng/mL |
| LOINC | [21582-2](https://loinc.org/21582-2/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in ng/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 1 | 15 | consensus | Asymptomatic controls, including HaT-positive persons (literature screen, consensus proposal) | [The Normal Range of Baseline Tryptase Should Be 1 to 15 ng/mL and Covers Healthy Individuals With HαT.](https://pubmed.ncbi.nlm.nih.gov/37572755/), Abstract | Basal serum tryptase normal range proposed by an expert consensus (ECNM and AIM). Based on 2 SD covering the 95% confidence interval across a literature screen. |
| clinical | warn | all | 8 | - | consensus | General population for basal serum tryptase (genetic context: HaT) | [The Normal Range of Baseline Tryptase Should Be 1 to 15 ng/mL and Covers Healthy Individuals With HαT.](https://pubmed.ncbi.nlm.nih.gov/37572755/), Consensus proposal paragraph | Genetically determined normal upper tryptase limit proposed by the consensus; below it the likelihood of HaT is rather low. Source band <= 8; stored as the warn low bound 8. |
| alt-1 | warn | all | - | 7.5 | cohort | Individuals with basal tryptase of 7.5 ng/mL or greater tested for HaT or with a disorder linked to elevated BST, Military Health System National Capital Region, 2018 to 2021 (retrospective, n=109) | [Elevated Basal Serum Tryptase: Disease Distribution and Variability in a Regional Health System.](https://pubmed.ncbi.nlm.nih.gov/35032694/), Introduction (study aims) | Study-defined high-normal basal tryptase category used in one retrospective health-system study. Not a guideline cut-off. Source band 7.5-11.5; stored as the warn high bound 7.5. Alternative to the default clinical warn range for the same population. |
| diagnostic | warn | systemic-mastocytosis | - | 15 | consensus | Patients with systemic mastocytosis (SM) compared with controls and HaT-positive persons | [The Normal Range of Baseline Tryptase Should Be 1 to 15 ng/mL and Covers Healthy Individuals With HαT.](https://pubmed.ncbi.nlm.nih.gov/37572755/), Consensus proposal paragraph | Clinically relevant tryptase cut-off proposed by the consensus. Most patients with systemic mastocytosis have tryptase above this value. Source kind: diagnostic. Source band >= 15; stored as the warn high bound 15. |
| diagnostic | warn | all | - | 11.5 | cohort | Individuals with basal tryptase of 7.5 ng/mL or greater tested for HaT or with a disorder linked to elevated BST, Military Health System National Capital Region, 2018 to 2021 (retrospective, n=109) | [Elevated Basal Serum Tryptase: Disease Distribution and Variability in a Regional Health System.](https://pubmed.ncbi.nlm.nih.gov/35032694/), Introduction | Elevated basal serum tryptase, the clinical definition cited in that study's introduction. The introduction lists HaT, mastocytosis, other myeloid neoplasms and CKD among disorders associated with BST elevation. Source kind: diagnostic. Source band >= 11.5; stored as the warn high bound 11.5. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
