# Bile Acids, Total (serum) (`bile-acids-total`)

Sum of serum bile acids. In pregnancy, raised levels with pruritus define intrahepatic cholestasis of pregnancy (ICP), and higher concentrations track with adverse perinatal outcome, especially stillbirth. Postprandial samples read higher than fasting samples.

| | |
|---|---|
| Category | [pregnancy](index.md#pregnancy) |
| Canonical unit | µmol/L |
| LOINC | - |
| Specimen | - |
| Aliases | - |

## Ranges

Bounds in µmol/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | pregnant, trimester 3, non-fasting | - | 18.3 | reference-interval-study | Third-trimester non-fasting reference range from 561 women of black, south Asian and white ethnicity (BJOG 2021 case-control and cohort study) (specimen: serum) | [Re-evaluating diagnostic thresholds for intrahepatic cholestasis of pregnancy: case-control and cohort study.](https://pubmed.ncbi.nlm.nih.gov/33586324/), Abstract, Results; [Re-evaluating diagnostic thresholds for intrahepatic cholestasis of pregnancy: case-control and cohort study.](https://pubmed.ncbi.nlm.nih.gov/33586324/), Abstract, Methods | Upper reference limit for non-fasting total serum bile acids in the third trimester (highest upper limit across the ethnic groups analysed). Non-fasting values rise after meals. |
| alt-1 | warn | pregnant, non-fasting, intrahepatic-cholestasis-of-pregnancy | - | 40 | cohort | Women with intrahepatic cholestasis of pregnancy, non-fasting sampling (BJOG 2021) (specimen: serum) | [Re-evaluating diagnostic thresholds for intrahepatic cholestasis of pregnancy: case-control and cohort study.](https://pubmed.ncbi.nlm.nih.gov/33586324/), Abstract, Conclusions | Severe ICP cut-off (40 µmol/L or more). Non-fasting sampling detected over 91% of severe cases in this study; fasting sampling detected 9%. Source kind: diagnostic. Source band >= 40; stored as the warn high bound 40. Alternative to the default bjog-2021-icp-thresholds warn range for the same population. |
| bjog-2021-icp-thresholds | warn | pregnant, non-fasting, intrahepatic-cholestasis-of-pregnancy | - | 19 | cohort | Women with suspected intrahepatic cholestasis of pregnancy, non-fasting sampling (BJOG 2021) (specimen: serum) | [Re-evaluating diagnostic thresholds for intrahepatic cholestasis of pregnancy: case-control and cohort study.](https://pubmed.ncbi.nlm.nih.gov/33586324/), Abstract, Conclusions | Proposed non-fasting threshold (19 µmol/L or more) that improves diagnostic accuracy because postprandial rise in normal pregnancy is accounted for. Compare with fasting thresholds. Source kind: diagnostic. Source band >= 19; stored as the warn high bound 19. |
| ovadia-2019-lancet | critical | pregnant, intrahepatic-cholestasis-of-pregnancy | - | 100 | cohort | Women with intrahepatic cholestasis of pregnancy, aggregate and individual patient data from 27 studies (singleton pregnancies) (specimen: serum) | [Association of adverse perinatal outcomes of intrahepatic cholestasis of pregnancy with biochemical markers: results of aggregate and individual patient data meta-analyses.](https://pubmed.ncbi.nlm.nih.gov/30773280/), Interpretation | Stillbirth risk is increased at 100 µmol/L or more in singleton pregnancies (stillbirth 3.44% vs 0.13% and 0.28% below 100 µmol/L). Most women with ICP are below this level; repeat testing is advised. Source band >= 100; stored as the critical high bound 100. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
