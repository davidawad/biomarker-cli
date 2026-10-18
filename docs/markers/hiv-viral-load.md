# HIV-1 RNA (Viral Load) (`hiv-viral-load`)

Amount of HIV-1 RNA in plasma. It is the main measure of HIV treatment response; the goal of antiretroviral therapy is to keep it below the assay threshold for suppression.

| | |
|---|---|
| Category | [infectious](index.md#infectious) |
| Canonical unit | copies/mL |
| LOINC | [20447-9](https://loinc.org/20447-9/) |
| Specimen | - |
| Aliases | `hiv viral load`, `hiv-1 rna pcr`, `hiv rna` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in copies/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | hiv | - | 200 | guideline | Adults with HIV receiving antiretroviral therapy | [Antiretroviral Drugs for Treatment and Prevention of HIV Infection in Adults: 2020 Recommendations of the International Antiviral Society-USA Panel.](https://pubmed.ncbi.nlm.nih.gov/33052386/), Full text, Virological failure definition | Virological failure: a confirmed HIV RNA strictly above 200 copies/mL on 2 consecutive measurements (boundary >200). Values of 200 or below are not failure by this definition. Source kind: decision. Source band >= 200; stored as the warn high bound 200. Alternative to the default ias-usa-2020 warn range for the same population. |
| ias-usa-2020 | warn | hiv | - | 50 | guideline | Adults with HIV receiving antiretroviral therapy | [Antiretroviral Drugs for Treatment and Prevention of HIV Infection in Adults: 2020 Recommendations of the International Antiviral Society-USA Panel.](https://pubmed.ncbi.nlm.nih.gov/33052386/), Section: monitoring after viral suppression | Warning band: after previous suppression, an HIV RNA above 50 copies/mL (boundary >50) should prompt a repeat test and a review of adherence and tolerability. Source band >= 50; stored as the warn high bound 50. |
| jama-2016-serodifferent-couples | warn | hiv | 200 | - | cohort | People with HIV on suppressive antiretroviral therapy in serodifferent couples (Undetectable = Untransmittable, U=U) | [Sexual Activity Without Condoms and Risk of HIV Transmission in Serodifferent Couples When the HIV-Positive Partner Is Using Suppressive Antiretroviral Therapy.](https://pubmed.ncbi.nlm.nih.gov/27404185/), Abstract, Objective | U=U threshold: HIV-1 RNA strictly less than 200 copies/mL (boundary <200; 200 itself is not included). Source kind: decision. Source band <= 200; stored as the warn low bound 200. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
