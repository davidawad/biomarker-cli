# Bone Mineral Density T-score (`dxa-t-score`)

Bone mineral density expressed as standard deviations from the mean of a young adult reference population, measured by DXA. Used to diagnose osteoporosis; a T-score of -2.5 or lower defines osteoporosis in postmenopausal women and men aged 50 and older.

| | |
|---|---|
| Category | [body](index.md#body) |
| Canonical unit | SD |
| LOINC | [38263-0](https://loinc.org/38263-0/) |
| Specimen | - |
| Aliases | `dxa t-score`, `bmd t-score`, `bone density t-score`, `bmd-t-score` |

## Ranges

No clinical reference range: No general clinical reference interval is cited; see the diagnostic, target and alternative ranges.

Bounds in SD. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| iscd-adult-positions | warn | post-menopausal | -2.5 | - | guideline | Postmenopausal women (DXA; lumbar spine, total hip or femoral neck) | [ISCD Official Positions: Adult Positions](https://iscd.org/learn/official-positions/adult-positions/), Diagnosis | Osteoporosis if T-score is -2.5 or less (boundary is less-than-or-equal). Site is the lumbar spine, total hip or femoral neck. Source kind: decision. Source band <= -2.5; stored as the warn low bound -2.5. |
| iscd-adult-positions | warn | male, age 50+ | -2.5 | - | guideline | Men aged 50 and older (DXA; lumbar spine, total hip or femoral neck) | [ISCD Official Positions: Adult Positions](https://iscd.org/learn/official-positions/adult-positions/), Diagnosis | Osteoporosis if T-score is -2.5 or less (boundary is less-than-or-equal). Source kind: decision. Source band <= -2.5; stored as the warn low bound -2.5. |
| iscd-adult-positions | warn | all | -2 | - | guideline | Premenopausal women and men under 50 (Z-score, not T-score); the quoted sentence does not state the age or menopausal status restriction | [ISCD Official Positions: Adult Positions](https://iscd.org/learn/official-positions/adult-positions/), Z-score interpretation | Z-score of -2.0 or lower is defined as 'below the expected range for age' (Z-score, not T-score). The ISCD text retrieved does not state the population for this band, so the premenopausal/under-50 applicability is not confirmed by the quote. Source kind: decision. Source band <= -2.0; stored as the warn low bound -2.0. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
