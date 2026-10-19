# International normalized ratio (`inr`)

Prothrombin time expressed as a ratio to a standard reference, so results are comparable across laboratories. Used to monitor vitamin K antagonist (warfarin) therapy; values above the target range indicate excess anticoagulation and bleeding risk, values below it indicate under-anticoagulation and thrombosis risk.

| | |
|---|---|
| Category | [coagulation](index.md#coagulation) |
| Canonical unit | ratio |
| LOINC | [6301-6](https://loinc.org/6301-6/) |
| Specimen | - |
| Aliases | - |
| Panels | [Liver health and MASLD](../panels/liver.md) |

## Ranges

Bounds in ratio. The most specific matching range of each set applies.

| Set | Kind | Applies to | Low | High | Evidence | Population | Citations | Note |
|---|---|---|---|---|---|---|---|---|
| clinical | reference | male, age 18-51 | 0.7 | 1.14 | reference-interval-study | 120 healthy male adults aged 18-50 in Debre Berhan town, Ethiopia, not anticoagulated (240 healthy adults in total) | [Coagulation parameters reference interval for adult population of Debre Berhan town, Northeast Ethiopia.](https://pubmed.ncbi.nlm.nih.gov/41275213/), Table 3 | Non-parametric 2.5th-97.5th percentiles, Sysmex CA-104 (reagent not named). Combined sexes 0.7-1.1. Ratio (INR), not seconds. |
| alt-1 | optimal | mechanical-mitral-valve, on-warfarin | 3 | 3 | guideline | adults with a mechanical mitral or double valve on long-term vitamin K antagonist therapy | [Antithrombotic and thrombolytic therapy for valvular disease: Antithrombotic Therapy and Prevention of Thrombosis, 9th ed: American College of Chest Physicians Evidence-Based Clinical Practice Guidelines.](https://pubmed.ncbi.nlm.nih.gov/22315272/), Abstract, Results | Guideline states a single target INR of 3.0 (Grade 2C), not a range; low and high are set equal to encode the point target. Source kind: therapeutic. Alternative to the default chest-2012-valve optimal range for the same population. |
| chest-2012-valve | optimal | mechanical-aortic-valve, on-warfarin | 2.5 | 2.5 | guideline | adults with a mechanical aortic valve on long-term vitamin K antagonist therapy | [Antithrombotic and thrombolytic therapy for valvular disease: Antithrombotic Therapy and Prevention of Thrombosis, 9th ed: American College of Chest Physicians Evidence-Based Clinical Practice Guidelines.](https://pubmed.ncbi.nlm.nih.gov/22315272/), Abstract, Results | Guideline states a single target INR of 2.5 (Grade 1B), not a range; low and high are set equal to encode the point target. The lower/upper bounds of a 2.5-3.5 style range are not stated in the retrieved text. Source kind: therapeutic. |
| chest-2012-vka | optimal | on-warfarin | 2 | 3 | guideline | adults on long-term vitamin K antagonist therapy (general indications, CHEST 2012 anticoagulation guideline) | [Evidence-based management of anticoagulant therapy: Antithrombotic Therapy and Prevention of Thrombosis, 9th ed: American College of Chest Physicians Evidence-Based Clinical Practice Guidelines.](https://pubmed.ncbi.nlm.nih.gov/22315259/), Abstract, Results | Strong recommendation (Grade 1B) for a target INR of 2.0 to 3.0 on VKA therapy. Indication-specific targets for mechanical valves are separate ranges; no reference (non-anticoagulated) INR range was retrieved. Source kind: therapeutic. |
| debre-berhan-2025 | reference | female, age 18-51 | 0.7 | 1.12 | reference-interval-study | 120 healthy female adults aged 18-50 in Debre Berhan town, Ethiopia, not anticoagulated (240 healthy adults in total) | [Coagulation parameters reference interval for adult population of Debre Berhan town, Northeast Ethiopia.](https://pubmed.ncbi.nlm.nih.gov/41275213/), Table 3 | Non-parametric 2.5th-97.5th percentiles, Sysmex CA-104 (reagent not named). Ratio (INR), not seconds. |

---

_Generated from catalog/ by `biomarker catalog docs`; do not edit by hand._
