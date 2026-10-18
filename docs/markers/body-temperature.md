# Body Temperature (`body-temperature`)

Core or oral body temperature. Normal values vary between individuals, and a raised temperature (fever) signals infection or inflammation.

| | |
|---|---|
| Category | [vitals](index.md#vitals) |
| Canonical unit | °C |
| LOINC | [8310-5](https://loinc.org/8310-5/) |
| Specimen | - |
| Aliases | `temperature`, `body temp`, `fever` |

## Ranges

Bounds in °C. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | all | 35.7 | 37.3 | reference-interval-study | Outpatient adults of a large academic hospital, USA (35,488 patients without infection or antibiotics; mean age 52.9 years) | [Individual differences in normal body temperature: longitudinal big data analysis of patient records.](https://pubmed.ncbi.nlm.nih.gov/29237616/), Abstract, Results | 95% range of individual-level baseline temperature. Temperature is lower with older age and higher in some groups, so individual values vary. The 99% range was 35.3-37.7 °C. |
| low-body-temp-review | critical | all | 35 | - | consensus | Adults, including older adults; general definition of hypothermia by core temperature | [Approach to Low Body Temperature or Mild Hypothermia in the Geriatric Population: A Narrative Review.](https://doi.org/10.14740/jocmr6490), Abstract | Hypothermia is defined as a temperature below 35 °C (boundary strictly below 35 °C, so 35.0 °C is not hypothermia by this definition). Described in the source as a medical emergency; the source is a narrative review, so this is a definitional cut-off. Source band <= 35; stored as the critical low bound 35. |
| mackowiak-jama-1992 | reference | age 0-41 | - | 37.7 | reference-interval-study | Healthy men and women aged 18 to 40 years, oral temperature measured with an electronic thermometer | [A critical appraisal of 98.6 degrees F, the upper limit of the normal body temperature, and other legacies of Carl Reinhold August Wunderlich.](https://pubmed.ncbi.nlm.nih.gov/1302471/), Abstract, Conclusions | Upper limit of the normal oral temperature range in healthy adults aged 40 years or younger, as proposed by the authors. The upper limit is 37.7 °C; the authors argue against the older 37.0 °C (98.6 °F) convention. |
| pyrexia-review-2016 | warn | all | - | 38 | consensus | Adults, including febrile critically ill patients (review of pyrexia definitions) | [Pyrexia: aetiology in the ICU.](https://pubmed.ncbi.nlm.nih.gov/27581757/), Abstract | Commonly used definition of fever (pyrexia) as 38.0 °C or greater. The source notes that definitions vary by population and thermometer, so this is a common convention rather than a single guideline cut-off. Source kind: decision. Source band >= 38.0; stored as the warn high bound 38.0. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
