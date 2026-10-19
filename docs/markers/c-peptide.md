# C-Peptide (`c-peptide`)

C-peptide is secreted in equimolar amounts with insulin and measures endogenous insulin secretion. Used to classify diabetes and guide treatment, interpreted differently for fasting, stimulated and urine tests.

| | |
|---|---|
| Category | [glycemic](index.md#glycemic) |
| Canonical unit | ng/mL |
| LOINC | - |
| Specimen | - |
| Aliases | - |
| Panels | [Diabetes and glycemic control](../panels/diabetes.md) |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in ng/mL. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| alt-1 | warn | non-fasting, diabetes | 1.812 | - | consensus | People with insulin-treated diabetes; stimulated (non-fasting random, post-glucagon or mixed-meal) C-peptide | [The clinical utility of C-peptide measurement in the care of patients with diabetes.](https://pubmed.ncbi.nlm.nih.gov/23413806/), Table 1 | Likely Type 1 diabetes / inability to achieve glycaemic control with non-insulin therapies (strictly below). Source unit nmol/L. Converted from the source's nmol/L (0.6) to ng/mL using factor 3.02. Source kind: diagnostic. Source band <= 1.812; stored as the warn low bound 1.812. Alternative to the default diagnostic warn range for the same population. |
| alt-1 | warn | fasting, diabetes | 0.755 | - | consensus | People with insulin-treated diabetes; fasting C-peptide | [The clinical utility of C-peptide measurement in the care of patients with diabetes.](https://pubmed.ncbi.nlm.nih.gov/23413806/), Table 1 | Likely Type 1 diabetes / inability to achieve glycaemic control with non-insulin therapies (strictly below). Source unit nmol/L. Converted from the source's nmol/L (0.25) to ng/mL using factor 3.02. Source kind: diagnostic. Source band <= 0.755; stored as the warn low bound 0.755. Alternative to the default diagnostic warn range for the same population. |
| alt-2 | warn | non-fasting, diabetes | - | 0.604 | consensus | People with insulin-treated diabetes > 3-5 years post-diagnosis; stimulated (non-fasting random, post-glucagon or mixed-meal) C-peptide | [The clinical utility of C-peptide measurement in the care of patients with diabetes.](https://pubmed.ncbi.nlm.nih.gov/23413806/), Table 1 | Suggests Type 2 or monogenic (MODY) diabetes in presumed Type 1 diabetes (strictly above). Source unit nmol/L. Converted from the source's nmol/L (0.2) to ng/mL using factor 3.02. Source kind: diagnostic. Source band >= 0.604; stored as the warn high bound 0.604. Alternative to the default diagnostic warn range for the same population. |
| alt-2 | warn | fasting, diabetes | - | 0.2416 | consensus | People with insulin-treated diabetes > 3-5 years post-diagnosis; fasting C-peptide | [The clinical utility of C-peptide measurement in the care of patients with diabetes.](https://pubmed.ncbi.nlm.nih.gov/23413806/), Table 1 | Suggests Type 2 or monogenic (MODY) diabetes in presumed Type 1 diabetes (strictly above). Source unit nmol/L. Converted from the source's nmol/L (0.08) to ng/mL using factor 3.02. Source kind: diagnostic. Source band >= 0.2416; stored as the warn high bound 0.2416. Alternative to the default diagnostic warn range for the same population. |
| alt-3 | warn | non-fasting, diabetes | - | 3.02 | consensus | People with insulin-treated young-onset diabetes at diagnosis; stimulated C-peptide | [The clinical utility of C-peptide measurement in the care of patients with diabetes.](https://pubmed.ncbi.nlm.nih.gov/23413806/), Table 1 | Consider MODY / Type 2 diabetes (strictly above). Source unit nmol/L. Converted from the source's nmol/L (1) to ng/mL using factor 3.02. Source kind: diagnostic. Source band >= 3.02; stored as the warn high bound 3.02. Alternative to the default diagnostic warn range for the same population. |
| alt-3 | warn | fasting, diabetes | - | 1.208 | consensus | People with insulin-treated young-onset diabetes at diagnosis; fasting C-peptide | [The clinical utility of C-peptide measurement in the care of patients with diabetes.](https://pubmed.ncbi.nlm.nih.gov/23413806/), Table 1 | Consider MODY / Type 2 diabetes (strictly above). Source unit nmol/L. Converted from the source's nmol/L (0.4) to ng/mL using factor 3.02. Source kind: diagnostic. Source band >= 1.208; stored as the warn high bound 1.208. Alternative to the default diagnostic warn range for the same population. |
| diagnostic | warn | non-fasting, diabetes | 0.604 | - | consensus | People with insulin-treated diabetes; stimulated (non-fasting random, post-glucagon or mixed-meal) C-peptide | [The clinical utility of C-peptide measurement in the care of patients with diabetes.](https://pubmed.ncbi.nlm.nih.gov/23413806/), Table 1 | Absolute insulin deficiency / absolute insulin requirement (strictly below). Source unit nmol/L (1 nmol/L = 3 ng/mL); numbers not converted. Converted from the source's nmol/L (0.2) to ng/mL using factor 3.02. Source kind: diagnostic. Source band <= 0.604; stored as the warn low bound 0.604. |
| diagnostic | warn | fasting, diabetes | 0.2416 | - | consensus | People with insulin-treated diabetes; fasting C-peptide | [The clinical utility of C-peptide measurement in the care of patients with diabetes.](https://pubmed.ncbi.nlm.nih.gov/23413806/), Table 1 | Absolute insulin deficiency / absolute insulin requirement (strictly below). Source unit nmol/L. Converted from the source's nmol/L (0.08) to ng/mL using factor 3.02. Source kind: diagnostic. Source band <= 0.2416; stored as the warn low bound 0.2416. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
