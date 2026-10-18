# Lactate (`lactate`)

Blood lactate, the product of anaerobic glycolysis. Raised values reflect tissue hypoperfusion or hypoxia (for example in sepsis) and predict mortality; very high values indicate severe shock or other lactic acidosis.

| | |
|---|---|
| Category | [organic-acid](index.md#organic-acid) |
| Canonical unit | mmol/L |
| LOINC | [2524-7](https://loinc.org/2524-7/) |
| Specimen | - |
| Aliases | - |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in mmol/L. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| j-sscg-2020 | warn | sepsis | - | 2 | guideline | Adults with sepsis in the Japanese guideline, septic shock definition (vasopressors to keep MAP at or above 65 mmHg, with lactate above this level despite fluids) | [The Japanese Clinical Practice Guidelines for Management of Sepsis and Septic Shock 2020 (J-SSCG 2020).](https://pubmed.ncbi.nlm.nih.gov/34433491/), Section on definitions, septic shock | Septic-shock lactate criterion in the Japanese 2020 guideline, the same 2 mmol/L level as Sepsis-3. Included as a second source for that threshold. Source kind: diagnostic. Source band >= 2; stored as the warn high bound 2. |
| lactate-kinetics-2016 | warn | critical-illness | - | 1.5 | consensus | Critically ill adults in a systematic review of blood lactate kinetics (observational and interventional studies) | [The value of blood lactate kinetics in critically ill patients: a systematic review.](https://pubmed.ncbi.nlm.nih.gov/27520452/), Introduction | Review statement that minor rises above this level are associated with higher mortality. For lactate, mEq/L equals mmol/L. The same review says the normal blood lactate concentration is around 1 mEq/L, a single value that is not encoded as an interval. Source band >= 1.5; stored as the warn high bound 1.5. |
| sepsis-3-2016 | warn | sepsis | - | 2 | consensus | Adults with sepsis and septic shock, measured after adequate fluid resuscitation (Sepsis-3 septic shock criteria) | [Developing a New Definition and Assessing New Clinical Criteria for Septic Shock: For the Third International Consensus Definitions for Sepsis and Septic Shock (Sepsis-3).](https://pubmed.ncbi.nlm.nih.gov/26903336/), Abstract, Conclusions | Septic-shock criterion in Sepsis-3 (with vasopressors); a level above 2 is the hyperlactataemia threshold used alongside hypotension. Source kind: diagnostic. Source band >= 2; stored as the warn high bound 2. |
| ssc-2012 | critical | sepsis | - | 4 | guideline | Adults with sepsis-induced tissue hypoperfusion requiring protocolised resuscitation (Surviving Sepsis Campaign 2012) | [Surviving Sepsis Campaign: international guidelines for management of severe sepsis and septic shock, 2012.](https://pubmed.ncbi.nlm.nih.gov/23361625/), Recommendation 1, Initial resuscitation | Resuscitation threshold: hypoperfusion is defined as persistent hypotension after fluid challenge or lactate at or above this level. Source band >= 4; stored as the critical high bound 4. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
