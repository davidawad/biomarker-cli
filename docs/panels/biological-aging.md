# Biological aging (`biological-aging`)

The nine routine blood markers that define Levine's Phenotypic Age (PhenoAge), plus the aging-clock outputs themselves.

## Core

| Marker | Reason | Citations |
|---|---|---|
| [Albumin](../markers/albumin.md) | PhenoAge input (liver); negative weight. | [An epigenetic biomarker of aging for lifespan and healthspan.](https://pubmed.ncbi.nlm.nih.gov/29676998/), Table 1 |
| [Creatinine](../markers/creatinine.md) | PhenoAge input (kidney). | [An epigenetic biomarker of aging for lifespan and healthspan.](https://pubmed.ncbi.nlm.nih.gov/29676998/), Table 1 |
| [Glucose](../markers/glucose.md) | PhenoAge input (metabolic), serum glucose. | [An epigenetic biomarker of aging for lifespan and healthspan.](https://pubmed.ncbi.nlm.nih.gov/29676998/), Table 1 |
| [C-Reactive Protein (standard assay)](../markers/crp.md) | PhenoAge input (inflammation), log-transformed CRP. | [An epigenetic biomarker of aging for lifespan and healthspan.](https://pubmed.ncbi.nlm.nih.gov/29676998/), Table 1 |
| [Lymphocytes (%)](../markers/lymphocytes-pct.md) | PhenoAge input (immune). | [An epigenetic biomarker of aging for lifespan and healthspan.](https://pubmed.ncbi.nlm.nih.gov/29676998/), Table 1 |
| [Mean Corpuscular Volume](../markers/mcv.md) | PhenoAge input (immune). | [An epigenetic biomarker of aging for lifespan and healthspan.](https://pubmed.ncbi.nlm.nih.gov/29676998/), Table 1 |
| [Red Cell Distribution Width](../markers/rdw.md) | PhenoAge input (immune); the largest single weight. | [An epigenetic biomarker of aging for lifespan and healthspan.](https://pubmed.ncbi.nlm.nih.gov/29676998/), Table 1 |
| [Alkaline Phosphatase](../markers/alp.md) | PhenoAge input (liver). | [An epigenetic biomarker of aging for lifespan and healthspan.](https://pubmed.ncbi.nlm.nih.gov/29676998/), Table 1 |
| [White Blood Cells](../markers/wbc.md) | PhenoAge input (immune). | [An epigenetic biomarker of aging for lifespan and healthspan.](https://pubmed.ncbi.nlm.nih.gov/29676998/), Table 1 |
| [PhenoAge (Levine Phenotypic Age)](../markers/phenoage.md) | Composite of the nine markers plus chronological age; predicts mortality. | [A new aging measure captures morbidity and mortality risk across diverse subpopulations from NHANES IV: A cohort study.](https://pubmed.ncbi.nlm.nih.gov/30596641/), Methods |

## Extended

| Marker | Reason | Citations |
|---|---|---|
| [GrimAge (DNA Methylation Clock)](../markers/grimage.md) | Epigenetic clock built from DNAm surrogates of plasma proteins and smoking pack-years; predicts lifespan. | [DNA methylation GrimAge strongly predicts lifespan and healthspan.](https://pubmed.ncbi.nlm.nih.gov/30669119/), Abstract |
| [GrimAge Acceleration](../markers/grimage-acceleration.md) | Chronological-age-adjusted GrimAge; associated with age-related conditions. | [DNA methylation GrimAge strongly predicts lifespan and healthspan.](https://pubmed.ncbi.nlm.nih.gov/30669119/), Abstract |
| [Horvath Epigenetic Clock (Epigenetic Age)](../markers/horvath-epigenetic-clock.md) | Multi-tissue DNA methylation age predictor (353 CpGs). | [DNA methylation age of human tissues and cell types.](https://pubmed.ncbi.nlm.nih.gov/24138928/), Abstract |
| [Klemera-Doubal Biological Age](../markers/klemera-doubal-biological-age.md) | Biomarker-based biological age algorithm; best mortality predictor among five algorithms compared in NHANES III. | [Modeling the rate of senescence: can estimated biological age predict mortality more accurately than chronological age?](https://pubmed.ncbi.nlm.nih.gov/23213031/), Abstract |


---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
