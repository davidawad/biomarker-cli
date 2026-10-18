# Specific IgE, Birch (White Birch) (`ige-birch`)

Serum IgE specific to birch pollen allergen, used to support the diagnosis of IgE-mediated birch pollen allergy. Interpret with symptoms and skin testing; a positive level does not by itself prove clinical allergy.

| | |
|---|---|
| Category | [allergy](index.md#allergy) |
| Canonical unit | kU/L |
| LOINC | [41874-9](https://loinc.org/41874-9/) |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in kU/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | warn | all | - | 1 | cohort | Schoolchildren (Norway) with allergic rhinoconjunctivitis, serum sIgE on the Siemens IMMULITE 2000 | [Establishing IMMULITE® 2000 cut-off values for serum allergen-specific immunoglobulin and exploring their relationship to exhaled nitric oxide.](https://pubmed.ncbi.nlm.nih.gov/24628428/), Table 3 | Screening cut-off giving 90% sensitivity for allergic rhinoconjunctivitis to birch pollen (sensitivity 90.4%, specificity 85.1%). Values below this argue against the diagnosis; values at or above it warrant further testing. Single observational study. Source band >= 1.0; stored as the warn high bound 1.0. |
| alt-1 | warn | all | - | 6.77 | cohort | Children aged 2-18 years with atopic dermatitis and birch sensitization (Korea), ImmunoCAP | [Clinical Characteristics of Oral Allergy Syndrome in Children with Atopic Dermatitis and Birch Sensitization: a Single Center Study.](https://pubmed.ncbi.nlm.nih.gov/30636944/), Abstract, Results | Cut-off associated with oral allergy syndrome (OAS) in birch-sensitized children with atopic dermatitis (sensitivity 55.6%, specificity 79.0%, AUC 0.653). Weak discrimination; this is an OAS association, not a birch allergy diagnostic threshold. Unit as stated by the source (kUA/L). Source band >= 6.77; stored as the warn high bound 6.77. Alternative to the default clinical warn range for the same population. |
| diagnostic | warn | all | - | 2.8 | cohort | Schoolchildren (Norway) with allergic rhinoconjunctivitis, serum sIgE on the Siemens IMMULITE 2000 | [Establishing IMMULITE® 2000 cut-off values for serum allergen-specific immunoglobulin and exploring their relationship to exhaled nitric oxide.](https://pubmed.ncbi.nlm.nih.gov/24628428/), Table 3 | Diagnostic cut-off giving 90% specificity for allergic rhinoconjunctivitis to birch pollen (sensitivity 80.2%, specificity 90.2%). Single observational study; not a guideline threshold. Source kind: diagnostic. Source band >= 2.8; stored as the warn high bound 2.8. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
