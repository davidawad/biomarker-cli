$ sh examples/people.sh
added person alex
added person sam

$ export BIOMARKER_PERSON=alex

$ biomarker import examples/showcase.csv
44 inserted, 0 replaced, 0 skipped, 0 invalid (44 rows)

$ biomarker latest -c lipid --units si
ID  PERSON  TAKEN_AT    MARKER             QUALIFIER  VALUE   UNIT    REF_LOW  REF_HIGH  FLAG    LAB
──  ──────  ──────────  ─────────────────  ─────────  ──────  ──────  ───────  ────────  ──────  ──────────────
18  alex    2024-03-05  lpa                           103.20  nmol/L              75.00  high    Synthetic Labs
38  alex    2025-03-11  apob                            0.71  g/L                  0.90  normal  Synthetic Labs
39  alex    2025-03-11  hdl-c                           1.47  mmol/L     1.03            normal  Synthetic Labs
37  alex    2025-03-11  ldl-c                           1.86  mmol/L               2.59  normal  Synthetic Labs
36  alex    2025-03-11  total-cholesterol               4.34  mmol/L               5.17  normal  Synthetic Labs
40  alex    2025-03-11  triglycerides                   1.08  mmol/L               1.69  normal  Synthetic Labs

$ biomarker flag --latest \
    --columns taken_at,marker,value,unit,ref_high,ref_flag,opt_high,opt_flag
TAKEN_AT    MARKER  VALUE   UNIT    REF_HIGH  REF_FLAG  OPT_HIGH  OPT_FLAG
──────────  ──────  ──────  ──────  ────────  ────────  ────────  ────────
2024-03-05  lpa     103.20  nmol/L     75.00  high         30.00  high

$ biomarker trend -m ldl,apob,hba1c,vitamin-d \
    --columns marker,unit,n,first,last,change_pct,slope_per_year,last_flag
MARKER     UNIT   N  FIRST   LAST   CHANGE_PCT  SLOPE_PER_YEAR  LAST_FLAG
─────────  ─────  ─  ──────  ─────  ──────────  ──────────────  ─────────
apob       mg/dL  3   98.00  71.00      -27.55          -17.25  normal
ldl-c      mg/dL  5  138.00  72.00      -47.83          -32.59  normal
hba1c      %      4    5.70   5.20       -8.77           -0.24  normal
vitamin-d  ng/mL  4   22.00  46.00      109.09           10.62  normal

$ biomarker diff 2023-02-14 2025-03-11 --changed \
    --columns marker,unit,from_value,to_value,change_pct,from_flag,to_flag
MARKER             UNIT   FROM_VALUE  TO_VALUE  CHANGE_PCT  FROM_FLAG  TO_FLAG
─────────────────  ─────  ──────────  ────────  ──────────  ─────────  ───────
hscrp              mg/L         2.40      0.60      -75.00  normal     normal
hdl-c              mg/dL       44.00     57.00       29.55  normal     normal
ldl-c              mg/dL      138.00     72.00      -47.83  high       normal
total-cholesterol  mg/dL      212.00    168.00      -20.75  high       normal
triglycerides      mg/dL      150.00     96.00      -36.00  normal     normal
glucose            mg/dL      100.89     86.48      -14.29  high       normal
hba1c              %            5.70      5.20       -8.77  high       normal
vitamin-d          ng/mL       22.00     46.00      109.09  low        normal

$ biomarker trend -m apob --format json | head -n 32
{
  "schema": "biomarker/v1",
  "kind": "trend",
  "generated_at": "2026-01-01T00:00:00Z",
  "count": 1,
  "unit_system": "canonical",
  "range_flavor": "reference",
  "windows": [
    "3m",
    "6m",
    "1y"
  ],
  "data": [
    {
      "person": "alex",
      "marker": "apob",
      "marker_name": "Apolipoprotein B",
      "category": "lipid",
      "unit": "mg/dL",
      "n": 3,
      "first_date": "2023-08-30",
      "last_date": "2025-03-11",
      "min": 71.0,
      "max": 98.0,
      "mean": 83.66666666666667,
      "median": 82.0,
      "stddev": 13.576941236277534,
      "first": 98.0,
      "last": 71.0,
      "change": -27.0,
      "change_pct": -27.55102040816326,
      "slope_per_year": -17.245010386047024,
