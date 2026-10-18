# Measles IgG (`measles-igg`)

Measles-specific IgG antibody concentration, measured by ELISA and expressed against the WHO 3rd International Standard. Used to judge immunity after vaccination or infection; the protective level depends on the assay method and cut-off used.

| | |
|---|---|
| Category | [infectious](index.md#infectious) |
| Canonical unit | mIU/mL |
| LOINC | - |
| Specimen | - |
| Aliases | `measles antibody`, `anti-measles igg`, `measles igg antibody` |

## Ranges

Bounds in mIU/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 150 | 200 | cohort | Children on biologic DMARD therapy in a preliminary case-control study | [Preliminary Case-Control Study of Antibody Response to Vaccines in Children on bDMARDs.](https://pubmed.ncbi.nlm.nih.gov/41300643/), Methods, serologic thresholds | Equivocal band for measles IgG by ELISA: 150 to 200 mIU/mL. Below 150 was classed seronegative and above 200 seropositive in this study (boundary for seropositivity strictly greater than 200). Band labels are the source's; the assay was not named in the retrieved text. Two-sided band stored as a reference range in its own set because it has no clinical reference range to orient it as a warn or critical cut point. |
| measles-elisa-200-2025 | warn | all | - | 200 | cohort | Residents of Ho Chi Minh City, Vietnam, in a 2024 serosurvey before a measles outbreak (method: elisa-manufacturer) | [Detection of Immunity Gap before Measles Outbreak, Ho Chi Minh City, Vietnam, 2024.](https://pubmed.ncbi.nlm.nih.gov/41017062/), Methods, serologic testing | Seropositivity cut-off for a commercial ELISA: IgG above 200 mIU/mL (boundary strictly greater than 200; sensitivity 99.0%, specificity 95.0% in this study). The cut-off is assay-specific and the source notes limited data on the protective threshold. Source kind: decision. Source band >= 200; stored as the warn high bound 200. |
| measles-prnt-120-2025 | warn | all | - | 120 | cohort | Vaccinated children in Kerala and Tamil Nadu, India (cross-sectional study of 684 children) (method: plaque-reduction-neutralisation) | [Low Measles Seropositivity in Vaccinated Children.](https://pubmed.ncbi.nlm.nih.gov/40864465/), Methods, neutralizing antibody assay; [Sustaining herd immunity against measles: Insights from a serological cohort study in an outbreak-free population.](https://pubmed.ncbi.nlm.nih.gov/42201515/), Discussion, paragraph on correlate of protection | Protective neutralising antibody threshold of 120 mIU/mL or higher, measured by the plaque reduction neutralisation test (PRNT, converted to mIU/mL against the WHO 3rd International Standard). This is the widely used correlate of protection; it is a neutralisation threshold, not an ELISA cut-off, and it should not be applied directly to commercial immunoassays. Source kind: decision. Source band >= 120; stored as the warn high bound 120. |
| siemens-enzygnost-measles-2026 | warn | all | - | 300 | consensus | Taiwan national measles elimination strategy; serology classification as described in the source (method: siemens-enzygnost) | [Taiwan's Strategy Toward Measles Elimination.](https://pubmed.ncbi.nlm.nih.gov/42042837/), Methods, serology classification | Siemens Enzygnost Anti-Measles Virus/IgG assay: positive above 300 mIU/mL (boundary strictly greater than 300); 150 to 300 mIU/mL is classed equivocal by the same assay. Method-specific; do not transfer to other ELISA kits. Source kind: decision. Source band >= 300; stored as the warn high bound 300. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
