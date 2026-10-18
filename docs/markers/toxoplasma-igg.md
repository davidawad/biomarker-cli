# Toxoplasma IgG (`toxoplasma-igg`)

Serum IgG antibody against Toxoplasma gondii, reported in IU/mL by immunoassay as negative, equivocal or positive. Tracked to establish past or latent infection, and during pregnancy or immunosuppression to assess risk. Expected result: negative (no prior exposure) or, if previously infected, positive; a positive IgG alone does not date infection, so IgM and IgG avidity testing are used with it.

| | |
|---|---|
| Category | [infectious](index.md#infectious) |
| Canonical unit | IU/mL |
| LOINC | [22580-5](https://loinc.org/22580-5/) |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in IU/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 6 | 9 | cohort | 384 routine-screening sera, Pitie-Salpetriere Hospital, Paris, tested with Platelia Toxo IgG and Western blot (method: platelia-toxo-igg) | [The impact of lowering the cut-off value on the sensitivity of the Platelia Elisa IgG (Bio-Rad) test for toxoplasmosis diagnosis.](https://pubmed.ncbi.nlm.nih.gov/26187780/), Abstract | Equivocal IgG on Platelia Toxo IgG (6-9 IU/mL), per the classes used in the cited study. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| alt-1 | warn | all | 6 | - | cohort | 384 routine-screening sera, Pitie-Salpetriere Hospital, Paris, tested with Platelia Toxo IgG and Western blot (method: platelia-toxo-igg) | [The impact of lowering the cut-off value on the sensitivity of the Platelia Elisa IgG (Bio-Rad) test for toxoplasmosis diagnosis.](https://pubmed.ncbi.nlm.nih.gov/26187780/), Abstract | Negative IgG on Platelia Toxo IgG (<6 IU/mL), per the classes used in the cited study. Source kind: diagnostic. Source band <= 6; stored as the warn low bound 6. Alternative to the default diagnostic warn range for the same population. |
| alt-1 | warn | pregnant | - | 187.6 | cohort | 757 pregnant women (843 sera), MEIA classified against Sabin-Feldman dye test and specific IgM (method: meia) | [Microparticle enzyme immunoassay (MEIA) for toxoplasma specific immunoglobulin G in comparison to the Sabin-Feldman dye test. A pilot study.](https://pubmed.ncbi.nlm.nih.gov/7709628/), Abstract | Group 3, acute infection, by MEIA IgG band. The source states these threshold values need confirmation in a larger prospective study. Source kind: diagnostic. Source band >= 187.6; stored as the warn high bound 187.6. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | all | - | 4.4 | cohort | 384 routine-screening sera, Pitie-Salpetriere Hospital, Paris, tested with Platelia Toxo IgG and Western blot (method: platelia-toxo-igg) | [The impact of lowering the cut-off value on the sensitivity of the Platelia Elisa IgG (Bio-Rad) test for toxoplasmosis diagnosis.](https://pubmed.ncbi.nlm.nih.gov/26187780/), Abstract | Seropositive (IgG positivity) on Platelia Toxo IgG; cut-off chosen in one study against Western blot, and lower than the manufacturer's cut-off. Source kind: diagnostic. Source band >= 4.4; stored as the warn high bound 4.4. |
| diagnostic | warn | pregnant | 5.2 | - | cohort | 757 pregnant women (843 sera), MEIA classified against Sabin-Feldman dye test and specific IgM (method: meia) | [Microparticle enzyme immunoassay (MEIA) for toxoplasma specific immunoglobulin G in comparison to the Sabin-Feldman dye test. A pilot study.](https://pubmed.ncbi.nlm.nih.gov/7709628/), Abstract | Group 1, no infection, by MEIA IgG band with dye test and IgM. The source states these threshold values need confirmation in a larger prospective study. Source kind: diagnostic. Source band <= 5.2; stored as the warn low bound 5.2. |
| toxoplasma-igg-bands-5-3-187-5 | reference | pregnant | 5.3 | 187.5 | cohort | 757 pregnant women (843 sera), MEIA classified against Sabin-Feldman dye test and specific IgM (method: meia) | [Microparticle enzyme immunoassay (MEIA) for toxoplasma specific immunoglobulin G in comparison to the Sabin-Feldman dye test. A pilot study.](https://pubmed.ncbi.nlm.nih.gov/7709628/), Abstract | Group 2, latent infection, by MEIA IgG band. The source states these threshold values need confirmation in a larger prospective study. Source kind: diagnostic. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
