$ sh examples/people.sh
added person alex
added person sam

$ export BIOMARKER_PERSON=alex

$ biomarker import examples/dashboard.xlsx --list-sheets
INDEX  NAME  ROWS  COLUMNS  DIMENSIONS
─────  ────  ────  ───────  ──────────
    1  Labs    23       11  A1:K23
    2  Body     5        6  A1:F5

$ biomarker import examples/dashboard.xlsx --mapping examples/dashboard-mapping.toml \
    --create-markers --dry-run
import examples/dashboard.xlsx (xlsx, sheet "Labs", transposed layout, header row 3)
matched (11):
  White Blood Cell Count (WBC)      -> wbc
  Red Blood Cell Count (RBC)        -> rbc
  Hemoglobin (Hgb)                  -> hemoglobin
  Mean Corpuscular Volume (MCV)     -> mcv
  Platelets                         -> platelets
  Low-Density Lipoprotein (LDL-C)   -> ldl-c
  High-Density Lipoprotein (HDL-C)  -> hdl-c
  Apolipoprotein B (ApoB)           -> apob
  Hemoglobin A1c                    -> hba1c
  ALT (SGPT)                        -> alt
  Thyroid Stimulating Hormone (TSH) -> tsh
created (3): urine-protein, urine-appearance, urine-wbc
skipped by mapping (2): Lipoprotein Particle Score, Control Sample
qualitative (11, skipped; use --qualitative store to keep them):
  2024-03-01  urine-protein: Negative
  2024-08-15  urine-protein: Negative
  2024-12-04  urine-protein: 1+ Abnormal
  2025-03-11  urine-protein: Negative
  2024-03-01  urine-appearance: Clear
  2024-08-15  urine-appearance: Clear
  2024-12-04  urine-appearance: Clear
  2024-03-01  urine-wbc: None seen
  2024-08-15  urine-wbc: 0-5
  2024-12-04  urine-wbc: 6-10 Abnormal
  2025-03-11  urine-wbc: None seen
date corrections (1):
  2024-12-02 -> 2024-12-04 (13 values)
reference ranges from the sheet (12):
  wbc: 3.4..10.8 10^3/µL
  rbc: 4.14..5.8 10^6/µL
  hemoglobin: 13..17.7 g/dL
  mcv: 79..97 fL
  platelets: 150..450 10^3/µL
  ldl-c: 0..99 mg/dL
  hdl-c: 39.. mg/dL
  apob: ..90 mg/dL
  hba1c: 4.8..5.6 %
  alt: 0..44 U/L
  tsh: 0.45..4.5 mIU/L
  urine-wbc: 0..5 /hpf
values per date:
  2024-03-01  11
  2024-08-15  11
  2024-12-04  10
  2025-03-11  11
dry run: 43 inserted, 0 replaced, 0 skipped, 0 invalid (16 rows)

$ biomarker import examples/dashboard.xlsx --sheet Body \
    --value-column 'weight=Weight (lbs.):lb' --value-column 'bmi=BMI kg/m²'
import examples/dashboard.xlsx (xlsx, sheet "Body", long layout, header row 1)
matched (2):
  Weight (lbs.) -> weight
  BMI kg/m²     -> bmi
values per date:
  2024-01-06  2
  2024-04-06  2
  2024-07-06  2
  2024-10-05  2
8 inserted, 0 replaced, 0 skipped, 0 invalid (4 rows)

$ biomarker latest -m weight,bmi --units si --columns taken_at,marker,value,unit
TAKEN_AT    MARKER  VALUE  UNIT
──────────  ──────  ─────  ─────
2024-10-05  bmi     24.60  kg/m²
2024-10-05  weight  80.01  kg
