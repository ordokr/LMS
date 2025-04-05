# LMS Integration Project - Central Reference Hub

_Last updated: 2025-04-05_

## 📊 Project Overview

```json
{
  "overall_status": "mid_development",
  "project_stats": {
    "foundation_complete": true,
    "model_implementation": "89%",
    "api_implementation": "0%",
    "ui_implementation": "90%",
    "test_coverage": "15%",
    "technical_debt": "0%"
  },
  "target_system": {
    "code_location": "C:\\Users\\Tim\\Desktop\\LMS"
  },
  "completion_forecasts": {
    "models": "2025-04-23",
    "api_endpoints": "2025-09-08",
    "ui_components": "2025-04-20",
    "entire_project": "2025-09-08"
  }
}
```

## 🔄 Source-to-Target Mapping

| Component | Source System | Source Location | Target Location | Status | Priority |
|-----------|---------------|-----------------|-----------------|--------|----------|
| User Model | Both | `canvas/.../user.rb` + `discourse/.../user.rb` | `src-tauri/src/models/user.rs` | ✅ 60% | High |
| Forum Topics | Discourse | `discourse/.../topic.rb` | `src-tauri/src/models/topic.rs` | ✅ 60% | High |
| Forum Posts | Discourse | `discourse/.../post.rb` | `src-tauri/src/models/post.rs` | ✅ 60% | High |
| Courses | Canvas | `canvas/.../course.rb` | `src-tauri/src/models/course.rs` | ✅ 55% | High |
| Forum API | Discourse | `discourse/.../topics_controller.rb` | `src-tauri/src/api/forum.rs` | ❌ 0% | High |
| Course API | Canvas | `canvas/.../courses_controller.rb` | `src-tauri/src/api/lms/courses.rs` | ❌ 0% | High |
| UI Components | Both | Multiple files | `src/components/` | ✅ 90% | High |


## 🔍 Integration Conflicts (Placeholder)

```json
{
  "model_conflicts": [],
  "route_conflicts": []
}
```

## 📋 Implementation Tasks

1. **Complete API Endpoint Implementation** (0/67 completed)
   - High Priority: Focus on areas like 'auth'
2. **Complete UI Component Implementation** (98/109 completed)
   - Implement components corresponding to new API endpoints
3. **Address Technical Debt** (Score: 0%)
   - Refactor 431 high complexity files
   - Improve test coverage (currently 15%)
4. **Integrate Key Systems** (e.g., Search, Notifications - if applicable)

## 📁 Project Directory Structure

```
/
 ├── shared/ # Other
 │  ├── api/ # Api
 │  ├── models/ # Models
 │  ├── src/ # Other
 │  │  └── models/ # Models
 ├── src/ # Other
 │  ├── bin/ # Other
 │  ├── components/ # Ui
 │  │  ├── admin/ # Other
 │  │  ├── auth/ # Other
 │  │  ├── common/ # Other
 │  │  ├── courses/ # Other
 │  │  ├── forum/ # Other
 │  │  │  ├── admin/ # Other
 │  │  │  ├── notifications/ # Other
 │  │  │  ├── topics/ # Other
 │  │  │  └── user/ # Other
 │  │  ├── layout/ # Other
 │  │  ├── lms/ # Other
 │  │  └── shared/ # Other
 │  ├── core/ # Other
 │  ├── features/ # Other
 │  │  ├── assignments/ # Other
 │  │  ├── courses/ # Other
 │  │  ├── dashboard/ # Other
 │  │  └── forum/ # Other
 │  ├── forum/ # Other
 │  ├── lms/ # Other
 │  ├── models/ # Models
 │  │  ├── forum/ # Other
 │  │  └── lms/ # Other
 │  ├── pages/ # Ui
 │  ├── services/ # Service
 │  │  └── api/ # Api
 │  ├── storage/ # Other
 │  ├── styles/ # Other
 │  ├── sync/ # Other
 │  └── utils/ # Utility
 ├── src-tauri/ # Other
 │  ├── capabilities/ # Other
 │  ├── gen/ # Other
 │  │  └── schemas/ # Other
 │  ├── icons/ # Other
 │  ├── migrations/ # Other
 │  ├── src/ # Other
 │  │  ├── api/ # Api
 │  │  │  └── lms/ # Other
 │  │  ├── bin/ # Other
 │  │  ├── core/ # Other
 │  │  ├── database/ # Other
 │  │  │  ├── migrations/ # Other
 │  │  │  ├── repositories/ # Other
 │  │  │  └── schema/ # Other
 │  │  ├── forum/ # Other
 │  │  ├── lms/ # Other
 │  │  │  ├── assignments/ # Other
 │  │  │  ├── courses/ # Other
 │  │  │  ├── grades/ # Other
 │  │  │  ├── models/ # Models
 │  │  │  ├── modules/ # Other
 │  │  │  └── users/ # Other
 │  │  ├── models/ # Models
 │  │  ├── repositories/ # Other
 │  │  ├── repository/ # Other
 │  │  ├── routes/ # Api
 │  │  ├── services/ # Service
 │  │  ├── sync/ # Other
 │  │  └── utils/ # Utility
 │  ├── target/ # Other
 │  │  ├── debug/ # Other
 │  │  │  ├── .fingerprint/ # Other
 │  │  │  │  ├── adler2-98f02fe56105c2c6/ # Other
 │  │  │  │  ├── ahash-06717a81a20a2427/ # Other
 │  │  │  │  ├── ahash-871117807450828e/ # Other
 │  │  │  │  ├── ahash-f2e06b1d41d4e6d7/ # Other
 │  │  │  │  ├── aho-corasick-8f4e991112313f51/ # Other
 │  │  │  │  ├── alloc-no-stdlib-4168b4549df78f01/ # Other
 │  │  │  │  ├── alloc-stdlib-41fab82aeccd3af7/ # Other
 │  │  │  │  ├── anyhow-281542f359c731cc/ # Other
 │  │  │  │  ├── anyhow-3bc200550c16a5a9/ # Other
 │  │  │  │  ├── anyhow-ffd5b94628bd2804/ # Other
 │  │  │  │  ├── autocfg-5be64e68343e3f85/ # Other
 │  │  │  │  ├── base64-0341be10d6ee5dbb/ # Other
 │  │  │  │  ├── bitflags-dc528b037d3ddc89/ # Other
 │  │  │  │  ├── bitflags-dfca5c4ffa4f1d59/ # Other
 │  │  │  │  ├── block-buffer-fce505b27849a28e/ # Other
 │  │  │  │  ├── brotli-1c9275f8101f720a/ # Other
 │  │  │  │  ├── brotli-decompressor-8018f8ddfa7e3624/ # Other
 │  │  │  │  ├── byteorder-d8a0d051f37c2508/ # Other
 │  │  │  │  ├── bytes-6e1b71233944c901/ # Other
 │  │  │  │  ├── camino-26d1099da9d8748f/ # Other
 │  │  │  │  ├── camino-360bd8503068cc1f/ # Other
 │  │  │  │  ├── camino-51646932955dafa0/ # Other
 │  │  │  │  ├── cargo-platform-b7e3c37bced6530b/ # Other
 │  │  │  │  ├── cargo_metadata-8568eac99275b1d9/ # Other
 │  │  │  │  ├── cargo_toml-8ba5596eeaba2d30/ # Other
 │  │  │  │  ├── cc-7247ae3d5011936f/ # Other
 │  │  │  │  ├── cfb-1b2c14017555ee84/ # Other
 │  │  │  │  ├── cfb-b994d267ecdce1a9/ # Other
 │  │  │  │  ├── cfg-if-120b7212e7bc72ec/ # Other
 │  │  │  │  ├── cfg_aliases-b192f1ea4dca3072/ # Other
 │  │  │  │  ├── convert_case-236a1cbec8478681/ # Other
 │  │  │  │  ├── cookie-06f52e869338fc07/ # Other
 │  │  │  │  ├── cookie-2cbde5d8d964d0d8/ # Other
 │  │  │  │  ├── cookie-5119a5ad25b5ce63/ # Other
 │  │  │  │  ├── cpufeatures-ad1b52aad28a379d/ # Other
 │  │  │  │  ├── crc32fast-e19b56ec8bff1c37/ # Other
 │  │  │  │  ├── crossbeam-channel-9b103e6dfaa79102/ # Other
 │  │  │  │  ├── crossbeam-utils-5991a5b67ea7faab/ # Other
 │  │  │  │  ├── crossbeam-utils-ba0c92835292d117/ # Other
 │  │  │  │  ├── crossbeam-utils-cbe266a46ed60de1/ # Other
 │  │  │  │  ├── crypto-common-bd640de6f68a642b/ # Other
 │  │  │  │  ├── cssparser-4bf3bd0a33d2a623/ # Other
 │  │  │  │  ├── cssparser-fc06a5c11cdc2673/ # Other
 │  │  │  │  ├── cssparser-fcf0c72fcf07e1a6/ # Other
 │  │  │  │  ├── cssparser-macros-f413ca938c900391/ # Other
 │  │  │  │  ├── ctor-582d1595635aa968/ # Other
 │  │  │  │  ├── darling-fdce484670ad0db0/ # Other
 │  │  │  │  ├── darling_core-2fbbe740715799f6/ # Other
 │  │  │  │  ├── darling_macro-4f1dfb6ec4f4b028/ # Other
 │  │  │  │  ├── deranged-ebe0836c0a5fe8ab/ # Other
 │  │  │  │  ├── derive_more-f43a1203e31595f1/ # Other
 │  │  │  │  ├── digest-165fe3b15ef32b94/ # Other
 │  │  │  │  ├── dirs-631823de042e1f01/ # Other
 │  │  │  │  ├── dirs-8a5ddafea5b50a06/ # Other
 │  │  │  │  ├── dirs-sys-8d9b80df5df41de9/ # Other
 │  │  │  │  ├── dirs-sys-e39bcf719cbc6b9e/ # Other
 │  │  │  │  ├── displaydoc-5ceb48239c3d02b0/ # Other
 │  │  │  │  ├── dpi-01704d92a222701a/ # Other
 │  │  │  │  ├── dtoa-0901a3ca3108dd04/ # Other
 │  │  │  │  ├── dtoa-short-9487b46171489175/ # Other
 │  │  │  │  ├── dunce-490ac6fa22527cc8/ # Other
 │  │  │  │  ├── dyn-clone-75b6dd3c7aa33e16/ # Other
 │  │  │  │  ├── embed-resource-90b6f8772fbbc4db/ # Other
 │  │  │  │  ├── equivalent-0351353d24c6c3e5/ # Other
 │  │  │  │  ├── erased-serde-845c4d103a05ea0a/ # Other
 │  │  │  │  ├── erased-serde-cef148b549bbb02e/ # Other
 │  │  │  │  ├── fallible-iterator-a07db4be72934cd9/ # Other
 │  │  │  │  ├── fallible-streaming-iterator-57d0912ce2967992/ # Other
 │  │  │  │  ├── fdeflate-7185481aa70d72db/ # Other
 │  │  │  │  ├── flate2-0abaee2210b49aff/ # Other
 │  │  │  │  ├── fnv-98890e3ff67f2430/ # Other
 │  │  │  │  ├── form_urlencoded-541512f1436da875/ # Other
 │  │  │  │  ├── form_urlencoded-5b653d67b5632efe/ # Other
 │  │  │  │  ├── futf-80f545c80fd8d883/ # Other
 │  │  │  │  ├── futures-core-f0c3102d4be2fd57/ # Other
 │  │  │  │  ├── futures-macro-70f3b4ef6f3e16c5/ # Other
 │  │  │  │  ├── futures-task-1c3fa5431198fd55/ # Other
 │  │  │  │  ├── futures-util-eb03dcf7e91c6de4/ # Other
 │  │  │  │  ├── fxhash-5d05d78bf976a1e0/ # Other
 │  │  │  │  ├── generic-array-41454881eed4ba1a/ # Other
 │  │  │  │  ├── generic-array-77239ea263932314/ # Other
 │  │  │  │  ├── generic-array-9860d6b1bdf0630c/ # Other
 │  │  │  │  ├── getrandom-27c1dd5c03a3ae4f/ # Other
 │  │  │  │  ├── getrandom-54ffde08c6eb1128/ # Other
 │  │  │  │  ├── getrandom-7bd03ff1b8f93637/ # Other
 │  │  │  │  ├── getrandom-96e35a732cf64637/ # Other
 │  │  │  │  ├── getrandom-afca8c5aea8b62a4/ # Other
 │  │  │  │  ├── getrandom-c3e788b2de1a2dce/ # Other
 │  │  │  │  ├── getrandom-c4e29e643ae8c79c/ # Other
 │  │  │  │  ├── getrandom-fb089228426e44c5/ # Other
 │  │  │  │  ├── glob-5f40d82da748bf51/ # Other
 │  │  │  │  ├── hashbrown-60c50b090eef0d8f/ # Other
 │  │  │  │  ├── hashbrown-a0662571c6c349c2/ # Other
 │  │  │  │  ├── hashbrown-ee8f70769bcf99ad/ # Other
 │  │  │  │  ├── hashlink-08ea427a9cf0f97f/ # Other
 │  │  │  │  ├── heck-954fec8511936345/ # Other
 │  │  │  │  ├── html5ever-8067af79ff5e3b26/ # Other
 │  │  │  │  ├── html5ever-9046645422760212/ # Other
 │  │  │  │  ├── html5ever-addb22be010ee5f4/ # Other
 │  │  │  │  ├── html5ever-e12deae1024ac731/ # Other
 │  │  │  │  ├── http-67027ffbacbd833d/ # Other
 │  │  │  │  ├── ico-1dab0dee03cdce02/ # Other
 │  │  │  │  ├── icu_collections-062f96a989adaa70/ # Other
 │  │  │  │  ├── icu_locid-e504cfccf92a9ce6/ # Other
 │  │  │  │  ├── icu_locid_transform-ee66226c8ae6d507/ # Other
 │  │  │  │  ├── icu_locid_transform_data-afa24778c8d7cbb2/ # Other
 │  │  │  │  ├── icu_locid_transform_data-b5d54492931b05c7/ # Other
 │  │  │  │  ├── icu_locid_transform_data-b8619c2bf6b71835/ # Other
 │  │  │  │  ├── icu_normalizer-b4fd7959611abaf1/ # Other
 │  │  │  │  ├── icu_normalizer_data-6e89247c8ee61ea6/ # Other
 │  │  │  │  ├── icu_normalizer_data-f5d826693c447c11/ # Other
 │  │  │  │  ├── icu_normalizer_data-fc03b7a19b8687f4/ # Other
 │  │  │  │  ├── icu_properties-4e55449bca9b84c3/ # Other
 │  │  │  │  ├── icu_properties_data-4143acc5be938195/ # Other
 │  │  │  │  ├── icu_properties_data-f05cb726c6c36131/ # Other
 │  │  │  │  ├── icu_properties_data-ffbd5f41721df6bd/ # Other
 │  │  │  │  ├── icu_provider-f95f84820ded329f/ # Other
 │  │  │  │  ├── icu_provider_macros-25c604628251c791/ # Other
 │  │  │  │  ├── ident_case-18c8dcfd426fd14f/ # Other
 │  │  │  │  ├── idna-f09255b86ab3b921/ # Other
 │  │  │  │  ├── idna_adapter-e376ecfd41f0d8eb/ # Other
 │  │  │  │  ├── indexmap-1faabd01804efc59/ # Other
 │  │  │  │  ├── indexmap-30064cd94b18dc15/ # Other
 │  │  │  │  ├── indexmap-5d518851cec85088/ # Other
 │  │  │  │  ├── indexmap-6774aa79ba03fe8e/ # Other
 │  │  │  │  ├── indexmap-a6df3bfe4110fd11/ # Other
 │  │  │  │  ├── indexmap-b690464c95b0f66b/ # Other
 │  │  │  │  ├── indexmap-bf10abdf167e789b/ # Other
 │  │  │  │  ├── infer-3799b39af7495959/ # Other
 │  │  │  │  ├── infer-8789aa5bc2bcdcb8/ # Other
 │  │  │  │  ├── itoa-04f76355b1fc7079/ # Other
 │  │  │  │  ├── itoa-92b00bb315392d07/ # Other
 │  │  │  │  ├── json-patch-059f4aa4022294e2/ # Other
 │  │  │  │  ├── json-patch-5474478bf446275a/ # Other
 │  │  │  │  ├── jsonptr-023ecbb36518e711/ # Other
 │  │  │  │  ├── jsonptr-f689219db3fd367f/ # Other
 │  │  │  │  ├── keyboard-types-77df573867a33983/ # Other
 │  │  │  │  ├── kuchikiki-b42ef11410f53cc0/ # Other
 │  │  │  │  ├── kuchikiki-f17c7ddcad74937d/ # Other
 │  │  │  │  ├── lazy_static-aa264dec8e156b7d/ # Other
 │  │  │  │  ├── libc-16ac275bd1c650e1/ # Service
 │  │  │  │  ├── libc-93e44df35748adb8/ # Service
 │  │  │  │  ├── libc-b1bd950dbccffc65/ # Service
 │  │  │  │  ├── libsqlite3-sys-9bbe9ac71a1a5f33/ # Service
 │  │  │  │  ├── libsqlite3-sys-9e40275a8af0a35c/ # Service
 │  │  │  │  ├── libsqlite3-sys-b150630aeef2f1aa/ # Service
 │  │  │  │  ├── litemap-34c4bc45f88d5ced/ # Other
 │  │  │  │  ├── lms-69210e7cd19e0e66/ # Other
 │  │  │  │  ├── lms-7c9bbf141c83030b/ # Other
 │  │  │  │  ├── lms-8750d2d73b2ecbbf/ # Other
 │  │  │  │  ├── lms_lib-5b26d527d7b50b16/ # Other
 │  │  │  │  ├── lock_api-6f116e7c0c710338/ # Other
 │  │  │  │  ├── lock_api-a78ea890c8369f83/ # Other
 │  │  │  │  ├── lock_api-e0fd97e91092b951/ # Other
 │  │  │  │  ├── log-72b178f73cb366a7/ # Other
 │  │  │  │  ├── mac-bd53eb1660b9f88a/ # Other
 │  │  │  │  ├── markup5ever-07c8efb6d4cd7f22/ # Other
 │  │  │  │  ├── markup5ever-43bbf8a86ea5ca0e/ # Other
 │  │  │  │  ├── markup5ever-6e6d60bb49f7572c/ # Other
 │  │  │  │  ├── markup5ever-a748bc2a1c3db0f6/ # Other
 │  │  │  │  ├── matches-bf92a90159188a6e/ # Other
 │  │  │  │  ├── memchr-105646b5756cb14a/ # Other
 │  │  │  │  ├── mime-64a84b31f0e207ba/ # Other
 │  │  │  │  ├── miniz_oxide-f621516bb9797304/ # Other
 │  │  │  │  ├── muda-d3cb32ced3f9af91/ # Other
 │  │  │  │  ├── new_debug_unreachable-29fcb247964d4e2a/ # Other
 │  │  │  │  ├── nodrop-27502c3a02f19342/ # Other
 │  │  │  │  ├── num-conv-25ee515b4446d7e9/ # Other
 │  │  │  │  ├── once_cell-ecc86b59685e8d56/ # Other
 │  │  │  │  ├── open-c0f3166a0f4d61ba/ # Other
 │  │  │  │  ├── option-ext-e4d4647d689901b6/ # Other
 │  │  │  │  ├── parking_lot-0813e500f3e7e2ad/ # Other
 │  │  │  │  ├── parking_lot_core-516ebb963f06c758/ # Other
 │  │  │  │  ├── parking_lot_core-984845d4f7980b69/ # Other
 │  │  │  │  ├── parking_lot_core-daa03bf66bdf8a2f/ # Other
 │  │  │  │  ├── percent-encoding-296d51829386c5ea/ # Other
 │  │  │  │  ├── percent-encoding-36355d0f5ec6ec70/ # Other
 │  │  │  │  ├── phf-07dd69e5ec4f3530/ # Other
 │  │  │  │  ├── phf-5096d45a3151c4f6/ # Other
 │  │  │  │  ├── phf-a2ec22d4845eca71/ # Other
 │  │  │  │  ├── phf-ac2238918588f686/ # Other
 │  │  │  │  ├── phf_codegen-8084638a5ea5c1e2/ # Other
 │  │  │  │  ├── phf_codegen-c11441c8c8cb7e31/ # Other
 │  │  │  │  ├── phf_generator-2078306ecfabf4b4/ # Other
 │  │  │  │  ├── phf_generator-7c507766349b57c3/ # Other
 │  │  │  │  ├── phf_generator-b29321a4220dccd6/ # Other
 │  │  │  │  ├── phf_macros-357151615e5f6e00/ # Other
 │  │  │  │  ├── phf_macros-b336334082349fad/ # Other
 │  │  │  │  ├── phf_shared-0495455b1a684a2e/ # Other
 │  │  │  │  ├── phf_shared-662d6527f3ed19f5/ # Other
 │  │  │  │  ├── phf_shared-e92aa4458f2fa7b8/ # Other
 │  │  │  │  ├── phf_shared-fb81fa80d638d506/ # Other
 │  │  │  │  ├── pin-project-lite-efc1287e85ca2a83/ # Other
 │  │  │  │  ├── pin-utils-8ff768994ab1a462/ # Other
 │  │  │  │  ├── pkg-config-dc03a79ea1264ab2/ # Other
 │  │  │  │  ├── png-dcd712c53e4f5658/ # Other
 │  │  │  │  ├── powerfmt-a21659ed8be10e5c/ # Other
 │  │  │  │  ├── ppv-lite86-f871e9dc957ba518/ # Other
 │  │  │  │  ├── precomputed-hash-49bcf51ca6d7e27c/ # Other
 │  │  │  │  ├── proc-macro-hack-040cc23240400206/ # Other
 │  │  │  │  ├── proc-macro-hack-13d670acc52eb777/ # Other
 │  │  │  │  ├── proc-macro-hack-29a601751312e838/ # Other
 │  │  │  │  ├── proc-macro2-631aca48eade5c45/ # Other
 │  │  │  │  ├── proc-macro2-e5e5052433162192/ # Other
 │  │  │  │  ├── proc-macro2-fd4439c1391c1df5/ # Other
 │  │  │  │  ├── quote-fb5425fcfea6edd8/ # Other
 │  │  │  │  ├── rand-5e78513713d96bc0/ # Other
 │  │  │  │  ├── rand-ea2d45209fd6b2c4/ # Other
 │  │  │  │  ├── rand_chacha-62a517f85dbc9e6b/ # Other
 │  │  │  │  ├── rand_chacha-a427262cb5c03b63/ # Other
 │  │  │  │  ├── rand_core-8b2e112f593ecf27/ # Other
 │  │  │  │  ├── rand_core-d3a113dde75fa0a6/ # Other
 │  │  │  │  ├── rand_pcg-b1542e49943bf419/ # Other
 │  │  │  │  ├── raw-window-handle-d5d6fd2f55cf9397/ # Other
 │  │  │  │  ├── regex-automata-b3135a381b13142c/ # Other
 │  │  │  │  ├── regex-d05050d7e3167dd1/ # Other
 │  │  │  │  ├── regex-syntax-dc3e929c3e31c4da/ # Other
 │  │  │  │  ├── rusqlite-459215c65f626adb/ # Other
 │  │  │  │  ├── rustc_version-228b9d8701835845/ # Other
 │  │  │  │  ├── ryu-6ac78a988ab9ad86/ # Other
 │  │  │  │  ├── same-file-03270b2e6fc16ddb/ # Other
 │  │  │  │  ├── same-file-7bd112d1a1dc0b7a/ # Other
 │  │  │  │  ├── schemars-10c7406afc1a31aa/ # Other
 │  │  │  │  ├── schemars-685e3ac89c359e05/ # Other
 │  │  │  │  ├── schemars-8c8c99346b12bf9d/ # Other
 │  │  │  │  ├── schemars_derive-81732cc9a7e00bd1/ # Other
 │  │  │  │  ├── scopeguard-4a1404f096e49870/ # Other
 │  │  │  │  ├── selectors-1dc674045f7f6d7c/ # Other
 │  │  │  │  ├── selectors-5f6db8115e811108/ # Other
 │  │  │  │  ├── selectors-cb9ef1f99d395211/ # Other
 │  │  │  │  ├── semver-0300d286d967957a/ # Other
 │  │  │  │  ├── semver-7120aa4fefb5cf0b/ # Other
 │  │  │  │  ├── semver-8ad401ef6d819402/ # Other
 │  │  │  │  ├── semver-91f6146674c69c43/ # Other
 │  │  │  │  ├── semver-c97dfeea256318a3/ # Other
 │  │  │  │  ├── semver-f0174dfafdde9797/ # Other
 │  │  │  │  ├── serde-0468fd937fd5e9ba/ # Other
 │  │  │  │  ├── serde-4b7c0bb640aabc5c/ # Other
 │  │  │  │  ├── serde-670f440499e905ce/ # Other
 │  │  │  │  ├── serde-67f1fa76f452c395/ # Other
 │  │  │  │  ├── serde-8767c121a099b839/ # Other
 │  │  │  │  ├── serde-e1d9460a071359e4/ # Other
 │  │  │  │  ├── serde-untagged-17ce8b8908b0c07e/ # Other
 │  │  │  │  ├── serde-untagged-380591fe27cbc8c4/ # Other
 │  │  │  │  ├── serde_derive-e62d014d8cfe9430/ # Other
 │  │  │  │  ├── serde_derive_internals-019ddb85945790fc/ # Other
 │  │  │  │  ├── serde_json-05aa2780d9f0df18/ # Other
 │  │  │  │  ├── serde_json-321e2af511c1d7e4/ # Other
 │  │  │  │  ├── serde_json-3ff06daefd83ff79/ # Other
 │  │  │  │  ├── serde_json-70ad7b8f0b2a96cd/ # Other
 │  │  │  │  ├── serde_json-99cd5de048fff4ef/ # Other
 │  │  │  │  ├── serde_json-f078b87621fd1bd0/ # Other
 │  │  │  │  ├── serde_repr-ae1a0f054e09765a/ # Other
 │  │  │  │  ├── serde_spanned-6343ec19a3aa9a36/ # Other
 │  │  │  │  ├── serde_spanned-beea9ac3d3ff6d94/ # Other
 │  │  │  │  ├── serde_with-4806d706f1717d9d/ # Other
 │  │  │  │  ├── serde_with-fdefa174357083a7/ # Other
 │  │  │  │  ├── serde_with_macros-c8a4a9b43e113e6a/ # Other
 │  │  │  │  ├── serialize-to-javascript-85efcc800eb5408c/ # Other
 │  │  │  │  ├── serialize-to-javascript-impl-0237e43769d0c6a1/ # Other
 │  │  │  │  ├── servo_arc-2a372508665aa6a9/ # Other
 │  │  │  │  ├── sha2-34d22f60a1dd79bc/ # Other
 │  │  │  │  ├── shlex-a918df6411f507de/ # Other
 │  │  │  │  ├── simd-adler32-fb0bc8735e3a2cbd/ # Other
 │  │  │  │  ├── siphasher-77157f26713148c2/ # Other
 │  │  │  │  ├── siphasher-fda3bf58e3c92d6d/ # Other
 │  │  │  │  ├── slab-0398d026498afef8/ # Other
 │  │  │  │  ├── slab-72f34d3e45a32922/ # Other
 │  │  │  │  ├── slab-fde0d11161d72c20/ # Other
 │  │  │  │  ├── smallvec-a63e90090a854e1e/ # Other
 │  │  │  │  ├── softbuffer-147bb227bd9d0ccd/ # Other
 │  │  │  │  ├── softbuffer-4d6ecbec2b1557da/ # Other
 │  │  │  │  ├── softbuffer-a24e52c7cd8efc6e/ # Other
 │  │  │  │  ├── stable_deref_trait-fd6a8db73e3da569/ # Other
 │  │  │  │  ├── string_cache-30b697dcab6707ea/ # Other
 │  │  │  │  ├── string_cache-63520310d64de144/ # Other
 │  │  │  │  ├── string_cache_codegen-088544efdbf0e75a/ # Other
 │  │  │  │  ├── strsim-9b129a8a078e578d/ # Other
 │  │  │  │  ├── syn-6e39e98972bfc421/ # Other
 │  │  │  │  ├── syn-76b300b3f9d51e1e/ # Other
 │  │  │  │  ├── syn-e22d705e2ebbff72/ # Other
 │  │  │  │  ├── syn-ffabd086f2e45bbc/ # Other
 │  │  │  │  ├── synstructure-3718f289b311a268/ # Other
 │  │  │  │  ├── tao-65faa7c715963808/ # Other
 │  │  │  │  ├── tauri-0dbeb2c1297e60eb/ # Other
 │  │  │  │  ├── tauri-453e7a120410de18/ # Other
 │  │  │  │  ├── tauri-codegen-60d6b57b184602f7/ # Other
 │  │  │  │  ├── tauri-dd870fc6585a5fc7/ # Other
 │  │  │  │  ├── tauri-macros-aaeeea1dea29b888/ # Other
 │  │  │  │  ├── tauri-plugin-34a0b8ca86f6f500/ # Other
 │  │  │  │  ├── tauri-plugin-opener-7714ff1633911954/ # Other
 │  │  │  │  ├── tauri-plugin-opener-9e669a20404a2da9/ # Other
 │  │  │  │  ├── tauri-plugin-opener-c8b052b1147e88d0/ # Other
 │  │  │  │  ├── tauri-runtime-415ac6dce89b2313/ # Other
 │  │  │  │  ├── tauri-runtime-dfe55144e7891c02/ # Other
 │  │  │  │  ├── tauri-runtime-ec94c7242261412b/ # Other
 │  │  │  │  ├── tauri-runtime-wry-380d931805ff2802/ # Other
 │  │  │  │  ├── tauri-runtime-wry-ae9c561008250b3c/ # Other
 │  │  │  │  ├── tauri-runtime-wry-e7f1fe20b3558682/ # Other
 │  │  │  │  ├── tauri-utils-dbf55d116f1589fe/ # Other
 │  │  │  │  ├── tauri-utils-ef465ec5a4951636/ # Other
 │  │  │  │  ├── tauri-winres-aa4b82e45498e65f/ # Other
 │  │  │  │  ├── tendril-a62f39c6a0830c7b/ # Other
 │  │  │  │  ├── thin-slice-389a0e92a0fd4a34/ # Other
 │  │  │  │  ├── thiserror-83c827a4f6cce979/ # Other
 │  │  │  │  ├── thiserror-8652639a0cc6527e/ # Other
 │  │  │  │  ├── thiserror-b74dba2b8289af0f/ # Other
 │  │  │  │  ├── thiserror-c9bd71dfae0e8985/ # Other
 │  │  │  │  ├── thiserror-cc3e84e7ebabeac7/ # Other
 │  │  │  │  ├── thiserror-fc1e639dc049fdc0/ # Other
 │  │  │  │  ├── thiserror-impl-10da01c7e20a164b/ # Other
 │  │  │  │  ├── thiserror-impl-b43e248f9324d879/ # Other
 │  │  │  │  ├── time-719d9347a22f9e7e/ # Other
 │  │  │  │  ├── time-core-defdcd232061ec7a/ # Other
 │  │  │  │  ├── time-macros-c0d5df18541a71e1/ # Other
 │  │  │  │  ├── tinystr-08dc436c896a2a74/ # Other
 │  │  │  │  ├── tokio-f5a6597eb5861361/ # Other
 │  │  │  │  ├── toml-61d1537883ced90d/ # Other
 │  │  │  │  ├── toml-ba5e6562aad5df7e/ # Other
 │  │  │  │  ├── toml_datetime-4dc0e870e9f4c789/ # Other
 │  │  │  │  ├── toml_datetime-6dcbdedf60e3d5b8/ # Other
 │  │  │  │  ├── toml_edit-702508fe2a4c13cc/ # Other
 │  │  │  │  ├── toml_edit-84db876cce58a05a/ # Other
 │  │  │  │  ├── typeid-6623b601df11d701/ # Other
 │  │  │  │  ├── typeid-82c9e1efd6aaf711/ # Other
 │  │  │  │  ├── typeid-ecf1989901f1bd21/ # Other
 │  │  │  │  ├── typenum-53455f76e3772f25/ # Other
 │  │  │  │  ├── typenum-b89a372d2f8e0a03/ # Other
 │  │  │  │  ├── typenum-c9d9165c9c15b87c/ # Other
 │  │  │  │  ├── unic-char-property-85491d16e676da4a/ # Other
 │  │  │  │  ├── unic-char-range-522d9bc42d8c76bb/ # Other
 │  │  │  │  ├── unic-common-b274c289710c37c2/ # Other
 │  │  │  │  ├── unic-ucd-ident-3d21c6c64c8758db/ # Other
 │  │  │  │  ├── unic-ucd-version-a8fe28d69c84739f/ # Other
 │  │  │  │  ├── unicode-ident-05b1ed5289651a81/ # Other
 │  │  │  │  ├── unicode-segmentation-b6580527f283b27a/ # Other
 │  │  │  │  ├── url-1351bf6c572047f7/ # Other
 │  │  │  │  ├── url-443f1d6f8c9bafeb/ # Other
 │  │  │  │  ├── urlpattern-a3ef14992e6f38c4/ # Other
 │  │  │  │  ├── urlpattern-a93fddf41634a605/ # Other
 │  │  │  │  ├── utf-8-c6f146d0dba3273f/ # Other
 │  │  │  │  ├── utf16_iter-27e504796cb8d100/ # Other
 │  │  │  │  ├── utf8_iter-470e50d0e67000d5/ # Other
 │  │  │  │  ├── uuid-2b9a6e7de99bcc9e/ # Other
 │  │  │  │  ├── uuid-38f6d6578dedb7f6/ # Other
 │  │  │  │  ├── vcpkg-066dd131759a4251/ # Other
 │  │  │  │  ├── version_check-cb5a7676e4932a43/ # Other
 │  │  │  │  ├── vswhom-9cfdcfe69b8410e2/ # Other
 │  │  │  │  ├── vswhom-sys-60a7ade249003848/ # Other
 │  │  │  │  ├── vswhom-sys-9babd224d815be5b/ # Other
 │  │  │  │  ├── vswhom-sys-f877ceac7c3daf06/ # Other
 │  │  │  │  ├── walkdir-8468591a70f6638d/ # Other
 │  │  │  │  ├── walkdir-e7f0061954d407b1/ # Other
 │  │  │  │  ├── webview2-com-c0abdcbbf0ce0fb2/ # Other
 │  │  │  │  ├── webview2-com-macros-1c4304396607b27c/ # Other
 │  │  │  │  ├── webview2-com-sys-0a13c6352e2eba0b/ # Other
 │  │  │  │  ├── webview2-com-sys-18eec868487bc866/ # Other
 │  │  │  │  ├── webview2-com-sys-c906e7784afe7f80/ # Other
 │  │  │  │  ├── winapi-util-06fb2afc9838daa6/ # Other
 │  │  │  │  ├── winapi-util-c5c63b95968a655d/ # Other
 │  │  │  │  ├── window-vibrancy-ff8fba78c5e4ff81/ # Other
 │  │  │  │  ├── windows-ce09e6ba83855168/ # Other
 │  │  │  │  ├── windows-collections-1ce224bef943a5f4/ # Other
 │  │  │  │  ├── windows-core-4c7473f6447b3c3a/ # Other
 │  │  │  │  ├── windows-future-9ddab3326dc73610/ # Other
 │  │  │  │  ├── windows-implement-f65e0eb9c15e90a4/ # Other
 │  │  │  │  ├── windows-interface-52ed9131706ce12d/ # Other
 │  │  │  │  ├── windows-link-7d8b2e9c4c517ee5/ # Other
 │  │  │  │  ├── windows-numerics-6d3802af97287b16/ # Other
 │  │  │  │  ├── windows-result-dc937b9c04ad74e9/ # Other
 │  │  │  │  ├── windows-strings-25810f1c5d216952/ # Other
 │  │  │  │  ├── windows-sys-90172794163dd43e/ # Other
 │  │  │  │  ├── windows-sys-b105dce82a018621/ # Other
 │  │  │  │  ├── windows-sys-eafb4782016284ec/ # Other
 │  │  │  │  ├── windows-targets-33a363ba09013758/ # Other
 │  │  │  │  ├── windows-targets-dd8cca008988cf75/ # Other
 │  │  │  │  ├── windows-version-521aa537262db6e3/ # Other
 │  │  │  │  ├── windows_x86_64_msvc-0b81e60db8677a5a/ # Other
 │  │  │  │  ├── windows_x86_64_msvc-3ccdbf08ae07867a/ # Other
 │  │  │  │  ├── windows_x86_64_msvc-e05cdbc621eda289/ # Other
 │  │  │  │  ├── windows_x86_64_msvc-f10211265d8162be/ # Other
 │  │  │  │  ├── windows_x86_64_msvc-f49fc5dda9a4ebdd/ # Other
 │  │  │  │  ├── windows_x86_64_msvc-fb5a8d7c417fe5b3/ # Other
 │  │  │  │  ├── winnow-4c9bb9d1791c1e80/ # Other
 │  │  │  │  ├── winreg-e8bf2629435c809e/ # Other
 │  │  │  │  ├── write16-0f669802e9093f18/ # Other
 │  │  │  │  ├── writeable-484bb341793a49d8/ # Other
 │  │  │  │  ├── wry-57ffdb0806929168/ # Other
 │  │  │  │  ├── wry-9395dc76598be150/ # Other
 │  │  │  │  ├── wry-f1071e62322461ae/ # Other
 │  │  │  │  ├── yoke-35bdb79916d0c737/ # Other
 │  │  │  │  ├── yoke-derive-857147a170f392e0/ # Other
 │  │  │  │  ├── zerocopy-23dfd521bd47ce33/ # Other
 │  │  │  │  ├── zerocopy-3c9564961073d2cf/ # Other
 │  │  │  │  ├── zerocopy-4fc916ac2a54efcb/ # Other
 │  │  │  │  ├── zerocopy-e84dbb4aa201f32d/ # Other
 │  │  │  │  ├── zerofrom-701456502742fd90/ # Other
 │  │  │  │  ├── zerofrom-derive-84ae567f3eec4cbd/ # Other
 │  │  │  │  ├── zerovec-c1634390851dd2ac/ # Other
 │  │  │  │  └── zerovec-derive-a13f8d0e73661c28/ # Other
 │  │  │  ├── deps/ # Other
 │  │  │  ├── examples/ # Other
 │  │  │  ├── incremental/ # Other
 │  │  │  │  ├── lms-26jonqsgir7ll/ # Other
 │  │  │  │  │  └── s-h60hftyrfa-0p6f9w8-working/ # Other
 │  │  │  │  ├── lms_lib-2044k10uw1tb0/ # Other
 │  │  │  │  │  └── s-h60hhoss7e-0ue1t2y-working/ # Other
 ├── style/ # Other
 ├── target/ # Other
 │  ├── debug/ # Other
 │  │  ├── .fingerprint/ # Other
 │  │  │  ├── adler2-169b89118f2b28c6/ # Other
 │  │  │  ├── ahash-3604983bcd8024ca/ # Other
 │  │  │  ├── ahash-66ae60e80ce355f3/ # Other
 │  │  │  ├── ahash-b6a4409ddb830b3e/ # Other
 │  │  │  ├── aho-corasick-32243e2a32d7f5b9/ # Other
 │  │  │  ├── aho-corasick-8f4e991112313f51/ # Other
 │  │  │  ├── aho-corasick-e315231722e3419f/ # Other
 │  │  │  ├── aho-corasick-eedab2244afccbf1/ # Other
 │  │  │  ├── alloc-no-stdlib-4168b4549df78f01/ # Other
 │  │  │  ├── alloc-stdlib-41fab82aeccd3af7/ # Other
 │  │  │  ├── allocator-api2-87381842b54c0315/ # Other
 │  │  │  ├── anyhow-281542f359c731cc/ # Other
 │  │  │  ├── anyhow-3bc200550c16a5a9/ # Other
 │  │  │  ├── anyhow-b077d1922ba42037/ # Other
 │  │  │  ├── anyhow-eb0ecae5a0db3aac/ # Other
 │  │  │  ├── anyhow-ffd5b94628bd2804/ # Other
 │  │  │  ├── async-recursion-17bca8526b7410c6/ # Other
 │  │  │  ├── async-recursion-716d8f5647a25e3c/ # Other
 │  │  │  ├── async-trait-00c89774a3bc8f89/ # Other
 │  │  │  ├── async-trait-cab19797ea180291/ # Other
 │  │  │  ├── async-trait-e2605eecc8e8877b/ # Other
 │  │  │  ├── atoi-a34d251a6565ee5c/ # Other
 │  │  │  ├── atoi-aa9e641fc2ce6155/ # Other
 │  │  │  ├── attribute-derive-caf2c4347b9457f7/ # Other
 │  │  │  ├── attribute-derive-ed46d0a9ee3c094a/ # Other
 │  │  │  ├── attribute-derive-macro-4334b2628b1d6dac/ # Other
 │  │  │  ├── attribute-derive-macro-d3d6feba6443b2af/ # Other
 │  │  │  ├── autocfg-5be64e68343e3f85/ # Other
 │  │  │  ├── axum-5d4d0425959f0e43/ # Other
 │  │  │  ├── axum-699c8991f204317f/ # Other
 │  │  │  ├── axum-core-1c32be56ef99338d/ # Other
 │  │  │  ├── axum-core-9f47c83853467bd4/ # Other
 │  │  │  ├── axum-core-d66b6a0aa88593a3/ # Other
 │  │  │  ├── axum-d2dc88afc7c6848e/ # Other
 │  │  │  ├── axum-macros-f1f4aeca0611bf40/ # Other
 │  │  │  ├── base64-0b3cd16d679ab49b/ # Other
 │  │  │  ├── base64-66a5ee4fa211dfd1/ # Other
 │  │  │  ├── base64-74f13c504a0a0f4e/ # Other
 │  │  │  ├── base64-8b5e603e5b19d66c/ # Other
 │  │  │  ├── base64-ca096900aac562c6/ # Other
 │  │  │  ├── base64-ed29d711b3799f73/ # Other
 │  │  │  ├── bcrypt-1119dce76710296f/ # Other
 │  │  │  ├── bitflags-a3b4274c99baa24d/ # Other
 │  │  │  ├── bitflags-dfca5c4ffa4f1d59/ # Other
 │  │  │  ├── block-buffer-1ac3449d05088418/ # Other
 │  │  │  ├── blowfish-dff47e4f6f4fa01a/ # Other
 │  │  │  ├── brotli-82b292736b206164/ # Other
 │  │  │  ├── brotli-decompressor-8018f8ddfa7e3624/ # Other
 │  │  │  ├── bstr-75f59a16f2ddbe60/ # Other
 │  │  │  ├── bumpalo-715c0932ee8c2e70/ # Other
 │  │  │  ├── bytemuck-63f16787e3925dcb/ # Other
 │  │  │  ├── byteorder-d8a0d051f37c2508/ # Other
 │  │  │  ├── bytes-0fb7751aec320709/ # Other
 │  │  │  ├── bytes-3231fe9448c90398/ # Other
 │  │  │  ├── bytes-6e1b71233944c901/ # Other
 │  │  │  ├── bytes-e2b3800749791570/ # Other
 │  │  │  ├── camino-8f01c79ffacff075/ # Other
 │  │  │  ├── camino-d8d9c365778f230d/ # Other
 │  │  │  ├── camino-ef2cc0eea88a8f90/ # Other
 │  │  │  ├── cargo_toml-edcb74402f016c5c/ # Other
 │  │  │  ├── cc-7247ae3d5011936f/ # Other
 │  │  │  ├── cfb-15f943fd590112cf/ # Other
 │  │  │  ├── cfb-879346e8e312e351/ # Other
 │  │  │  ├── cfg-if-120b7212e7bc72ec/ # Other
 │  │  │  ├── cfg-if-55dc4ad9a293f01d/ # Other
 │  │  │  ├── cfg-if-f2cfec39f226e007/ # Other
 │  │  │  ├── chrono-2e4342b044799eb4/ # Other
 │  │  │  ├── ciborium-2a98482a7c7488a3/ # Other
 │  │  │  ├── ciborium-7eb8aede39d2e884/ # Other
 │  │  │  ├── ciborium-io-54c7460c5ccd0245/ # Other
 │  │  │  ├── ciborium-io-a917e011f6344691/ # Other
 │  │  │  ├── ciborium-ll-8d685ca81ea977bd/ # Other
 │  │  │  ├── ciborium-ll-92a8b2794947796c/ # Other
 │  │  │  ├── cipher-071791f2a705719b/ # Other
 │  │  │  ├── collection_literals-144cd87503907866/ # Other
 │  │  │  ├── color_quant-1186ddca2d2a7d66/ # Other
 │  │  │  ├── config-7c4ebffc145e038a/ # Configuration
 │  │  │  ├── config-f0c3d3f0db55d0ec/ # Configuration
 │  │  │  ├── console_error_panic_hook-aff8e8437330c185/ # Other
 │  │  │  ├── console_error_panic_hook-bb488d61d2a25bff/ # Other
 │  │  │  ├── const_format-1ef653da48688adf/ # Other
 │  │  │  ├── const_format-6601d66ef37d49a8/ # Other
 │  │  │  ├── const_format-ea17c5fb494a0f1e/ # Other
 │  │  │  ├── const_format_proc_macros-88f5a895ffcfa35b/ # Other
 │  │  │  ├── const_format_proc_macros-aa8548b0b8e28066/ # Other
 │  │  │  ├── convert_case-236a1cbec8478681/ # Other
 │  │  │  ├── convert_case-a1c61e86c47701f5/ # Other
 │  │  │  ├── cpufeatures-74077e1018e3c4ca/ # Other
 │  │  │  ├── crc-catalog-c873fc02293324ce/ # Other
 │  │  │  ├── crc-fbbbe845a020ed4a/ # Other
 │  │  │  ├── crc32fast-3eeb57e99fd7cc8c/ # Other
 │  │  │  ├── crossbeam-channel-9b103e6dfaa79102/ # Other
 │  │  │  ├── crossbeam-deque-194856561dd33bdb/ # Other
 │  │  │  ├── crossbeam-epoch-6a158d0ced0b517e/ # Other
 │  │  │  ├── crossbeam-queue-55945ec322b1834c/ # Other
 │  │  │  ├── crossbeam-utils-5991a5b67ea7faab/ # Other
 │  │  │  ├── crossbeam-utils-ba0c92835292d117/ # Other
 │  │  │  ├── crossbeam-utils-cbe266a46ed60de1/ # Other
 │  │  │  ├── crypto-common-d42c52aa3ccb8bc0/ # Other
 │  │  │  ├── cssparser-1ec2ba2a2049a080/ # Other
 │  │  │  ├── cssparser-fc06a5c11cdc2673/ # Other
 │  │  │  ├── cssparser-fcf0c72fcf07e1a6/ # Other
 │  │  │  ├── cssparser-macros-91b55a841ace323b/ # Other
 │  │  │  ├── ctor-37eb6caa4cc9647f/ # Other
 │  │  │  ├── darling-144568c40420beee/ # Other
 │  │  │  ├── darling_core-7d44f4c25c0e9c36/ # Other
 │  │  │  ├── darling_macro-1ba5875dc7d1851e/ # Other
 │  │  │  ├── deranged-ebe0836c0a5fe8ab/ # Other
 │  │  │  ├── derive-where-76a33f2c8c68664b/ # Other
 │  │  │  ├── derive-where-8f5f405e84a53cce/ # Other
 │  │  │  ├── derive_more-8259dc5ad49fab53/ # Other
 │  │  │  ├── digest-b3b75552319c8f50/ # Other
 │  │  │  ├── dirs-next-2b28d7ef359827ce/ # Other
 │  │  │  ├── dirs-next-2f0a7c9477b7fb52/ # Other
 │  │  │  ├── dirs-sys-next-578f7a21b6a44acb/ # Other
 │  │  │  ├── dirs-sys-next-c861e29b0f172393/ # Other
 │  │  │  ├── displaydoc-665fbd00331359b2/ # Other
 │  │  │  ├── displaydoc-90cc8d30c680d015/ # Other
 │  │  │  ├── displaydoc-df7033b12d2f2e13/ # Other
 │  │  │  ├── dotenvy-b431d1017425cb11/ # Other
 │  │  │  ├── drain_filter_polyfill-9018d989ef6c25eb/ # Other
 │  │  │  ├── drain_filter_polyfill-dadcf096a8a6fbe4/ # Other
 │  │  │  ├── dtoa-0901a3ca3108dd04/ # Other
 │  │  │  ├── dtoa-short-9487b46171489175/ # Other
 │  │  │  ├── dunce-490ac6fa22527cc8/ # Other
 │  │  │  ├── either-1d96f53eee066c54/ # Other
 │  │  │  ├── either-a42a57c60727e805/ # Other
 │  │  │  ├── either-cabfc35271718a1b/ # Other
 │  │  │  ├── either-e80bdec8c784aa18/ # Other
 │  │  │  ├── embed-resource-8138fe7ceebe740f/ # Other
 │  │  │  ├── encoding_rs-4f3ce6607bfa70b2/ # Other
 │  │  │  ├── encoding_rs-c40de45fdf9b9712/ # Other
 │  │  │  ├── equivalent-0351353d24c6c3e5/ # Other
 │  │  │  ├── equivalent-4044c7ec24f556c4/ # Other
 │  │  │  ├── equivalent-4321c324b46f9ca6/ # Other
 │  │  │  ├── event-listener-fb74e70ac33a0240/ # Other
 │  │  │  ├── fastrand-9bedf595bf0fc5c3/ # Other
 │  │  │  ├── fdeflate-7185481aa70d72db/ # Other
 │  │  │  ├── filetime-9744765ff30500f3/ # Other
 │  │  │  ├── flate2-97f603d553bc78f6/ # Other
 │  │  │  ├── flate2-ead08328e4b7e75e/ # Other
 │  │  │  ├── flume-543be3b193b7abb1/ # Other
 │  │  │  ├── flume-f8becdf55c5d948c/ # Other
 │  │  │  ├── fnv-0a0b205450d4c0fa/ # Other
 │  │  │  ├── fnv-98890e3ff67f2430/ # Other
 │  │  │  ├── form_urlencoded-1c94341dff998aec/ # Other
 │  │  │  ├── form_urlencoded-2d84e94501081c48/ # Other
 │  │  │  ├── form_urlencoded-da3d6eb3563d5673/ # Other
 │  │  │  ├── futf-80f545c80fd8d883/ # Other
 │  │  │  ├── futures-37bfd7ad38bce4e3/ # Other
 │  │  │  ├── futures-channel-927559467d2b9178/ # Other
 │  │  │  ├── futures-channel-d28f9025a2098656/ # Other
 │  │  │  ├── futures-channel-fc933f5ba1427b0e/ # Other
 │  │  │  ├── futures-core-0b407bdde9496332/ # Other
 │  │  │  ├── futures-core-b17e2c2a87306059/ # Other
 │  │  │  ├── futures-executor-11dbf2d0ca4d8186/ # Other
 │  │  │  ├── futures-executor-36f31b815f5d8be1/ # Other
 │  │  │  ├── futures-executor-5c1b2d199f0a74f6/ # Other
 │  │  │  ├── futures-executor-e581721b331b8d1e/ # Other
 │  │  │  ├── futures-f46b6abf9430903f/ # Other
 │  │  │  ├── futures-intrusive-5638f1c79e84719d/ # Other
 │  │  │  ├── futures-io-5273fe5482d10a60/ # Other
 │  │  │  ├── futures-io-535de18222887846/ # Other
 │  │  │  ├── futures-io-b16ac9f488b8d9f8/ # Other
 │  │  │  ├── futures-macro-2929125cccbbe85b/ # Other
 │  │  │  ├── futures-macro-2dd326e37bcf92bb/ # Other
 │  │  │  ├── futures-macro-478111430d377e6c/ # Other
 │  │  │  ├── futures-sink-606e9d172833c945/ # Other
 │  │  │  ├── futures-sink-aa7e954255f4c0c6/ # Other
 │  │  │  ├── futures-sink-cc57c04ea1de971c/ # Other
 │  │  │  ├── futures-task-0318a42d10c9618f/ # Other
 │  │  │  ├── futures-task-1c3fa5431198fd55/ # Other
 │  │  │  ├── futures-util-155adfd73570481a/ # Other
 │  │  │  ├── futures-util-c2f42f3466fe746f/ # Other
 │  │  │  ├── futures-util-f751a587573f83b9/ # Other
 │  │  │  ├── futures-util-fba47f40bd46c604/ # Other
 │  │  │  ├── fxhash-5d05d78bf976a1e0/ # Other
 │  │  │  ├── generic-array-11f847d6dda6b8d4/ # Other
 │  │  │  ├── generic-array-684a68f00a2647b4/ # Other
 │  │  │  ├── generic-array-9860d6b1bdf0630c/ # Other
 │  │  │  ├── getrandom-27c1dd5c03a3ae4f/ # Other
 │  │  │  ├── getrandom-55ce9492b8d02544/ # Other
 │  │  │  ├── getrandom-6dd0dcc646dd2d5b/ # Other
 │  │  │  ├── getrandom-74b2f356b43b2a0b/ # Other
 │  │  │  ├── getrandom-7bd03ff1b8f93637/ # Other
 │  │  │  ├── getrandom-96e35a732cf64637/ # Other
 │  │  │  ├── getrandom-afca8c5aea8b62a4/ # Other
 │  │  │  ├── getrandom-bd02d4b7181501f9/ # Other
 │  │  │  ├── getrandom-c4e29e643ae8c79c/ # Other
 │  │  │  ├── getrandom-fb089228426e44c5/ # Other
 │  │  │  ├── glob-5f40d82da748bf51/ # Other
 │  │  │  ├── globset-413b1652718453ea/ # Other
 │  │  │  ├── gloo-net-9356874fd95fa3bc/ # Other
 │  │  │  ├── gloo-net-9b14bf23227a27cd/ # Other
 │  │  │  ├── gloo-net-b568b754b6e494f0/ # Other
 │  │  │  ├── gloo-net-e76e9d210d3fa7e3/ # Other
 │  │  │  ├── gloo-utils-35e7d2ef8a596282/ # Other
 │  │  │  ├── gloo-utils-452b8edeb96ac0ef/ # Other
 │  │  │  ├── gloo-utils-cc8125a8e267338a/ # Other
 │  │  │  ├── gloo-utils-ed0d616f4f223ad1/ # Other
 │  │  │  ├── h2-53133a8cb3e9cc5b/ # Other
 │  │  │  ├── h2-8f6b02faeb22ed0d/ # Other
 │  │  │  ├── h2-b5d352e72f3e3452/ # Other
 │  │  │  ├── half-430671db4fc3c874/ # Other
 │  │  │  ├── half-e263c0d21706477b/ # Other
 │  │  │  ├── hashbrown-0e794e7a0bf5e03f/ # Other
 │  │  │  ├── hashbrown-6b062f8820ad5646/ # Other
 │  │  │  ├── hashbrown-a0662571c6c349c2/ # Other
 │  │  │  ├── hashbrown-e3fa001cd7f9fc8c/ # Other
 │  │  │  ├── hashbrown-ee8f70769bcf99ad/ # Other
 │  │  │  ├── hashlink-f5997af8c28eb0a3/ # Other
 │  │  │  ├── headers-cd91e581d8aab652/ # Other
 │  │  │  ├── headers-core-e8ff021df93c6352/ # Other
 │  │  │  ├── heck-f384cf787246e8f2/ # Other
 │  │  │  ├── heck-fa499b1356116690/ # Other
 │  │  │  ├── hex-451a953863df013d/ # Other
 │  │  │  ├── html-escape-35fc8979ae17d675/ # Other
 │  │  │  ├── html-escape-6299f42d2ed08bd2/ # Other
 │  │  │  ├── html-escape-93cf3e7f53adc39e/ # Other
 │  │  │  ├── html5ever-8067af79ff5e3b26/ # Other
 │  │  │  ├── html5ever-a1daf1b291754b37/ # Other
 │  │  │  ├── html5ever-addb22be010ee5f4/ # Other
 │  │  │  ├── html5ever-f91715633266183d/ # Other
 │  │  │  ├── http-46b0fb6317b19cbc/ # Other
 │  │  │  ├── http-add1964abf6c0369/ # Other
 │  │  │  ├── http-body-4934e2b498b81684/ # Other
 │  │  │  ├── http-body-d8352e0ddca6bb53/ # Other
 │  │  │  ├── http-body-f7772095c0b270a0/ # Other
 │  │  │  ├── http-d7982a394e156816/ # Other
 │  │  │  ├── http-range-54eb5084376b826f/ # Other
 │  │  │  ├── http-range-header-fdd0690e0edbffd2/ # Other
 │  │  │  ├── httparse-7a9cf2ec73ddb78e/ # Other
 │  │  │  ├── httparse-8d2f3f63d5f468be/ # Other
 │  │  │  ├── httparse-a79e6f9b497909d6/ # Other
 │  │  │  ├── httparse-e3d68bb13f761d35/ # Other
 │  │  │  ├── httpdate-9257db76ad17ff29/ # Other
 │  │  │  ├── httpdate-ab5970b01025fed0/ # Other
 │  │  │  ├── hyper-45c225412a32b785/ # Other
 │  │  │  ├── hyper-b9c1cf58ef8caf24/ # Other
 │  │  │  ├── hyper-dcd3ca5b947788d3/ # Other
 │  │  │  ├── hyper-tls-3a84a450f9317f94/ # Other
 │  │  │  ├── ico-c9c0bf2bc5ae87e2/ # Other
 │  │  │  ├── icu_collections-48cebc5d95121001/ # Other
 │  │  │  ├── icu_collections-867cc58a690d385d/ # Other
 │  │  │  ├── icu_collections-97313e2db94e18e5/ # Other
 │  │  │  ├── icu_locid-0b56ea985f7ceea1/ # Other
 │  │  │  ├── icu_locid-39d66873339e91cd/ # Other
 │  │  │  ├── icu_locid-b3f3d6bc7a3d28c4/ # Other
 │  │  │  ├── icu_locid_transform-0e04fcbaa6f6cfa3/ # Other
 │  │  │  ├── icu_locid_transform-0fd5aa0232ffab51/ # Other
 │  │  │  ├── icu_locid_transform-8208159676f8b2cc/ # Other
 │  │  │  ├── icu_locid_transform_data-afa24778c8d7cbb2/ # Other
 │  │  │  ├── icu_locid_transform_data-b5d54492931b05c7/ # Other
 │  │  │  ├── icu_locid_transform_data-b8619c2bf6b71835/ # Other
 │  │  │  ├── icu_locid_transform_data-f11d6fccc210325b/ # Other
 │  │  │  ├── icu_normalizer-3e88530d899edd33/ # Other
 │  │  │  ├── icu_normalizer-44a0700f2e00a366/ # Other
 │  │  │  ├── icu_normalizer-a1e22c9d9fca010f/ # Other
 │  │  │  ├── icu_normalizer_data-2f7332df0030ff7e/ # Other
 │  │  │  ├── icu_normalizer_data-6e89247c8ee61ea6/ # Other
 │  │  │  ├── icu_normalizer_data-f5d826693c447c11/ # Other
 │  │  │  ├── icu_normalizer_data-fc03b7a19b8687f4/ # Other
 │  │  │  ├── icu_properties-819f5cec05c3a342/ # Other
 │  │  │  ├── icu_properties-94ee3a77c01a4666/ # Other
 │  │  │  ├── icu_properties-be326a5cd629f084/ # Other
 │  │  │  ├── icu_properties_data-4143acc5be938195/ # Other
 │  │  │  ├── icu_properties_data-b4f88f7cd177eed6/ # Other
 │  │  │  ├── icu_properties_data-f05cb726c6c36131/ # Other
 │  │  │  ├── icu_properties_data-ffbd5f41721df6bd/ # Other
 │  │  │  ├── icu_provider-9503ca2e5a3c2b4b/ # Other
 │  │  │  ├── icu_provider-bd3dab6634f23457/ # Other
 │  │  │  ├── icu_provider-f66bb2c7e4b63bc3/ # Other
 │  │  │  ├── icu_provider_macros-8694e7dfcae0213b/ # Other
 │  │  │  ├── icu_provider_macros-ad2e21990cb6c508/ # Other
 │  │  │  ├── icu_provider_macros-e15dd17b34210ade/ # Other
 │  │  │  ├── ident_case-18c8dcfd426fd14f/ # Other
 │  │  │  ├── idna-0308ba9afbef1fbf/ # Other
 │  │  │  ├── idna-05c935ca3caa2b53/ # Other
 │  │  │  ├── idna-7bbccd5b45147dc9/ # Other
 │  │  │  ├── idna_adapter-5a5966b9ba69bda7/ # Other
 │  │  │  ├── idna_adapter-5aae6949c30fea69/ # Other
 │  │  │  ├── idna_adapter-f98b6fe6e927e85b/ # Other
 │  │  │  ├── ignore-7ab62f5933cf2c6f/ # Other
 │  │  │  ├── image-5366fc6655f2024b/ # Other
 │  │  │  ├── indexmap-18a7b12cbf5755da/ # Other
 │  │  │  ├── indexmap-5d518851cec85088/ # Other
 │  │  │  ├── indexmap-9b1020f8878f3260/ # Other
 │  │  │  ├── indexmap-a52a37a2cd98165b/ # Other
 │  │  │  ├── indexmap-af328707eb37665a/ # Other
 │  │  │  ├── indexmap-bf10abdf167e789b/ # Other
 │  │  │  ├── indexmap-c0c87c228416f735/ # Other
 │  │  │  ├── indexmap-d276190d5d9f7a3d/ # Other
 │  │  │  ├── indexmap-f1be7af915674a67/ # Other
 │  │  │  ├── infer-90eff4e4a51d9e8d/ # Other
 │  │  │  ├── infer-b85b613b52904623/ # Other
 │  │  │  ├── inout-8382f59fcf7c9f61/ # Other
 │  │  │  ├── instant-7da65afc8f736b5b/ # Other
 │  │  │  ├── interpolator-b81147300ef86329/ # Other
 │  │  │  ├── inventory-2267d50707db3855/ # Other
 │  │  │  ├── inventory-b8a0dadc5e7ab454/ # Other
 │  │  │  ├── ipnet-1b997c301036a51e/ # Other
 │  │  │  ├── ipnet-7ce64db73d9ff15f/ # Other
 │  │  │  ├── itertools-49547f3226fa3e04/ # Other
 │  │  │  ├── itertools-b886d36c205615ad/ # Other
 │  │  │  ├── itertools-ea69b477ea94c70e/ # Other
 │  │  │  ├── itoa-04f76355b1fc7079/ # Other
 │  │  │  ├── itoa-53eb272c646ddda3/ # Other
 │  │  │  ├── itoa-92b00bb315392d07/ # Other
 │  │  │  ├── js-sys-342c029295a27d12/ # Other
 │  │  │  ├── js-sys-4313f8243e32c256/ # Other
 │  │  │  ├── json-patch-6998a0d25768fab2/ # Other
 │  │  │  ├── json-patch-79e3f9af4e889442/ # Other
 │  │  │  ├── jsonptr-8684ea331fc9f16b/ # Other
 │  │  │  ├── jsonptr-ee1985f0197cfffd/ # Other
 │  │  │  ├── jsonwebtoken-45f4250697efd873/ # Other
 │  │  │  ├── kuchikiki-14fdbffba6221c13/ # Other
 │  │  │  ├── kuchikiki-f65814c008ba2458/ # Other
 │  │  │  ├── lazy_static-9478afca47aaf4bc/ # Other
 │  │  │  ├── lazy_static-aa264dec8e156b7d/ # Other
 │  │  │  ├── leptos-1a4fc6dd74f86732/ # Other
 │  │  │  ├── leptos-de475f7c8eeaf69b/ # Other
 │  │  │  ├── leptos_config-7e44cd3e6146e75a/ # Other
 │  │  │  ├── leptos_config-94ebebe4a1144792/ # Other
 │  │  │  ├── leptos_dom-894c3b00a22172c2/ # Other
 │  │  │  ├── leptos_dom-b4171035d3932530/ # Other
 │  │  │  ├── leptos_hot_reload-76e87163e49b1300/ # Other
 │  │  │  ├── leptos_hot_reload-e4fec802aa483ad6/ # Other
 │  │  │  ├── leptos_macro-0c12a145460be2dc/ # Other
 │  │  │  ├── leptos_macro-558e129d34f94d92/ # Other
 │  │  │  ├── leptos_meta-0caaaa28f4ad094c/ # Other
 │  │  │  ├── leptos_meta-d5ecc9c26e786b34/ # Other
 │  │  │  ├── leptos_reactive-0f501b48b59ba1ec/ # Other
 │  │  │  ├── leptos_reactive-b2a7bb36b4b3201c/ # Other
 │  │  │  ├── leptos_router-35e461ba61e5d451/ # Other
 │  │  │  ├── leptos_router-cdc72ef8544e58e2/ # Other
 │  │  │  ├── leptos_server-65d21a25fa3eadca/ # Other
 │  │  │  ├── leptos_server-6ec9b98cc1e4fdfc/ # Other
 │  │  │  ├── libc-16ac275bd1c650e1/ # Service
 │  │  │  ├── libc-93e44df35748adb8/ # Service
 │  │  │  ├── libc-b1bd950dbccffc65/ # Service
 │  │  │  ├── libsqlite3-sys-4a687ed9ebebe44a/ # Service
 │  │  │  ├── libsqlite3-sys-5a05d2946c530e2a/ # Service
 │  │  │  ├── libsqlite3-sys-d9413d695d304a98/ # Service
 │  │  │  ├── linear-map-171abb9f46e9c8be/ # Other
 │  │  │  ├── linear-map-1f9297d90aade3fc/ # Other
 │  │  │  ├── litemap-34c4bc45f88d5ced/ # Other
 │  │  │  ├── litemap-e0b40ff19a461144/ # Other
 │  │  │  ├── lms-09ba4c1003447e6e/ # Other
 │  │  │  ├── lms-56c6fccfa1c07def/ # Other
 │  │  │  ├── lms-5b67674bd525f4c1/ # Other
 │  │  │  ├── lms-9038f2cbc5cc83c5/ # Other
 │  │  │  ├── lms-e3cc833abc400bb2/ # Other
 │  │  │  ├── lms-ecebd7a77e0e97b6/ # Other
 │  │  │  ├── lms-shared-f9fc617411f10bbc/ # Other
 │  │  │  ├── lms-ui-2a89b1800081111c/ # Other
 │  │  │  ├── lms-ui-4e4f093b2254e77a/ # Other
 │  │  │  ├── lms-ui-b144e185e04f0a90/ # Other
 │  │  │  ├── lock_api-6f116e7c0c710338/ # Other
 │  │  │  ├── lock_api-8cc838e8fee50cad/ # Other
 │  │  │  ├── lock_api-a3991c86cde2b339/ # Other
 │  │  │  ├── lock_api-a78ea890c8369f83/ # Other
 │  │  │  ├── lock_api-e0fd97e91092b951/ # Other
 │  │  │  ├── log-2ffaff091ebe6623/ # Other
 │  │  │  ├── log-72b178f73cb366a7/ # Other
 │  │  │  ├── log-9c7587dae26395d6/ # Other
 │  │  │  ├── log-c603d8f956e435cc/ # Other
 │  │  │  ├── mac-bd53eb1660b9f88a/ # Other
 │  │  │  ├── manyhow-2eab7bcf3a0f79eb/ # Other
 │  │  │  ├── manyhow-7aa5db5c2fa1bcce/ # Other
 │  │  │  ├── manyhow-macros-0a90216946df0e62/ # Other
 │  │  │  ├── manyhow-macros-2ba676196f83cf1b/ # Other
 │  │  │  ├── markup5ever-1c1cf100c0b023b6/ # Other
 │  │  │  ├── markup5ever-3440f7a964b731df/ # Other
 │  │  │  ├── markup5ever-9d49e90275983f1e/ # Other
 │  │  │  ├── markup5ever-b7cef07a61f182f9/ # Other
 │  │  │  ├── matchers-e5a15fcee43e7c53/ # Other
 │  │  │  ├── matches-bf92a90159188a6e/ # Other
 │  │  │  ├── matchit-b1ee7651a4c4350d/ # Other
 │  │  │  ├── memchr-105646b5756cb14a/ # Other
 │  │  │  ├── memchr-629c94ca16dc5e8a/ # Other
 │  │  │  ├── mime-64a84b31f0e207ba/ # Other
 │  │  │  ├── mime-c7044bb89b712223/ # Other
 │  │  │  ├── mime_guess-00d5ee55446283a3/ # Other
 │  │  │  ├── mime_guess-33c9edd60a17fcc4/ # Other
 │  │  │  ├── mime_guess-562e0f9b74ef8d8b/ # Other
 │  │  │  ├── minimal-lexical-c41aeaecdac4f0b2/ # Other
 │  │  │  ├── minimal-lexical-ca24b02f2b0ddc15/ # Other
 │  │  │  ├── miniz_oxide-384c2e8c9ce6d422/ # Other
 │  │  │  ├── miniz_oxide-6e0066e1564fab78/ # Other
 │  │  │  ├── mio-7175e77e6bc0752a/ # Other
 │  │  │  ├── mio-cd32166ce457e7ff/ # Other
 │  │  │  ├── mio-fc11af2075a7a966/ # Other
 │  │  │  ├── native-tls-0021ef0193193b0e/ # Other
 │  │  │  ├── native-tls-421dbf3c2e4365d3/ # Other
 │  │  │  ├── native-tls-8efbfbb71dd8b154/ # Other
 │  │  │  ├── new_debug_unreachable-29fcb247964d4e2a/ # Other
 │  │  │  ├── nodrop-27502c3a02f19342/ # Other
 │  │  │  ├── nom-78d8028cc1a84c94/ # Other
 │  │  │  ├── nom-9f3dec5c2a16e0c4/ # Other
 │  │  │  ├── nu-ansi-term-49d40126f3ae0d80/ # Other
 │  │  │  ├── num-bigint-7ff8ed45d066c4f6/ # Other
 │  │  │  ├── num-conv-25ee515b4446d7e9/ # Other
 │  │  │  ├── num-integer-ac35da0b91736fab/ # Other
 │  │  │  ├── num-traits-01634bea37f895be/ # Other
 │  │  │  ├── num-traits-0da2d3294c2a2ce3/ # Other
 │  │  │  ├── num-traits-2e43b29421481b3b/ # Other
 │  │  │  ├── num-traits-816acc0a3d4965e5/ # Other
 │  │  │  ├── num-traits-cbd49beb48bccffa/ # Other
 │  │  │  ├── num-traits-dc2df436b8df16b0/ # Other
 │  │  │  ├── num-traits-e5ee96478b9d9bfe/ # Other
 │  │  │  ├── num-traits-efdd366233be74ce/ # Other
 │  │  │  ├── num-traits-f552c0eb93004a13/ # Other
 │  │  │  ├── once_cell-c993e18e2f83a89f/ # Other
 │  │  │  ├── once_cell-ecc86b59685e8d56/ # Other
 │  │  │  ├── once_cell-fc27bbd0d3f73aab/ # Other
 │  │  │  ├── open-1219240ef713df0a/ # Other
 │  │  │  ├── overload-aa143794bcccf766/ # Other
 │  │  │  ├── pad-adapter-22b9dfeb809a66c6/ # Other
 │  │  │  ├── pad-adapter-ca2316cd98a3f9b7/ # Other
 │  │  │  ├── parking_lot-0813e500f3e7e2ad/ # Other
 │  │  │  ├── parking_lot-6666162500f9b66e/ # Other
 │  │  │  ├── parking_lot-a3199e6446f910fa/ # Other
 │  │  │  ├── parking_lot_core-3073ce1519c8277e/ # Other
 │  │  │  ├── parking_lot_core-516ebb963f06c758/ # Other
 │  │  │  ├── parking_lot_core-984845d4f7980b69/ # Other
 │  │  │  ├── parking_lot_core-af97a8bcdf0aa8fc/ # Other
 │  │  │  ├── parking_lot_core-daa03bf66bdf8a2f/ # Other
 │  │  │  ├── paste-8437bbe3ed31f599/ # Other
 │  │  │  ├── paste-ecb76f8a15ec9acb/ # Other
 │  │  │  ├── paste-f72d3230470b18a0/ # Other
 │  │  │  ├── pathdiff-132cb5f7c101d71d/ # Other
 │  │  │  ├── pathdiff-24ff702934341982/ # Other
 │  │  │  ├── pem-1f6bccdf17826fb4/ # Other
 │  │  │  ├── percent-encoding-36355d0f5ec6ec70/ # Other
 │  │  │  ├── percent-encoding-cbc1c9ff1c96fdf9/ # Other
 │  │  │  ├── phf-47188dbd4329f642/ # Other
 │  │  │  ├── phf-5096d45a3151c4f6/ # Other
 │  │  │  ├── phf-9e8210282390b4a6/ # Other
 │  │  │  ├── phf-ac2238918588f686/ # Other
 │  │  │  ├── phf_codegen-809a7ffa956c6516/ # Other
 │  │  │  ├── phf_codegen-a9660bb4df8d1fec/ # Other
 │  │  │  ├── phf_generator-82fdad06a7f46735/ # Other
 │  │  │  ├── phf_generator-cda570c695ccb845/ # Other
 │  │  │  ├── phf_generator-f5234b9e283a6fac/ # Other
 │  │  │  ├── phf_macros-4bc031fcaf98038c/ # Other
 │  │  │  ├── phf_macros-8b913c572cae83a7/ # Other
 │  │  │  ├── phf_shared-0495455b1a684a2e/ # Other
 │  │  │  ├── phf_shared-662d6527f3ed19f5/ # Other
 │  │  │  ├── phf_shared-e92aa4458f2fa7b8/ # Other
 │  │  │  ├── phf_shared-fb81fa80d638d506/ # Other
 │  │  │  ├── pin-project-34b915475dc9cbed/ # Other
 │  │  │  ├── pin-project-373bfd214cd1e884/ # Other
 │  │  │  ├── pin-project-a4abc7ed55873204/ # Other
 │  │  │  ├── pin-project-internal-1e60a517a8f62da3/ # Other
 │  │  │  ├── pin-project-internal-49259c3245cf50e9/ # Other
 │  │  │  ├── pin-project-internal-578f12e493c11a1d/ # Other
 │  │  │  ├── pin-project-lite-833b3a3dc313ae34/ # Other
 │  │  │  ├── pin-project-lite-efc1287e85ca2a83/ # Other
 │  │  │  ├── pin-project-lite-f2833ec83a908a9e/ # Other
 │  │  │  ├── pin-utils-8c2b952dd72e675c/ # Other
 │  │  │  ├── pin-utils-8ff768994ab1a462/ # Other
 │  │  │  ├── pkg-config-dc03a79ea1264ab2/ # Other
 │  │  │  ├── png-11db113929919f88/ # Other
 │  │  │  ├── powerfmt-a21659ed8be10e5c/ # Other
 │  │  │  ├── ppv-lite86-2e7dc31f6ab76eb0/ # Other
 │  │  │  ├── precomputed-hash-49bcf51ca6d7e27c/ # Other
 │  │  │  ├── prettyplease-116ab5c2a47a359d/ # Other
 │  │  │  ├── prettyplease-bc50a29add1d7d7d/ # Other
 │  │  │  ├── prettyplease-c5fb0cb51c7f4279/ # Other
 │  │  │  ├── prettyplease-fe6a88baf3cba35e/ # Other
 │  │  │  ├── proc-macro-error-05ed638be18ed77c/ # Other
 │  │  │  ├── proc-macro-error-attr-4ea97242da3750ef/ # Other
 │  │  │  ├── proc-macro-error-attr-6b8e876f3a396be0/ # Other
 │  │  │  ├── proc-macro-error-attr-c2fd4a1abd3d8b67/ # Other
 │  │  │  ├── proc-macro-error-attr-e56efbabfe172bd4/ # Other
 │  │  │  ├── proc-macro-error-deb41f66dc6959cc/ # Other
 │  │  │  ├── proc-macro-error-f7d59b72fc506b16/ # Other
 │  │  │  ├── proc-macro-error-fb595d23dd56c578/ # Other
 │  │  │  ├── proc-macro-hack-040cc23240400206/ # Other
 │  │  │  ├── proc-macro-hack-13d670acc52eb777/ # Other
 │  │  │  ├── proc-macro-hack-29a601751312e838/ # Other
 │  │  │  ├── proc-macro-utils-b5722277dff3b9f8/ # Other
 │  │  │  ├── proc-macro-utils-ffb14233ff7ee80f/ # Other
 │  │  │  ├── proc-macro2-18fe0881f273dffc/ # Other
 │  │  │  ├── proc-macro2-1b748c91458da0e8/ # Other
 │  │  │  ├── proc-macro2-4ee7dbdaa33ecf4a/ # Other
 │  │  │  ├── proc-macro2-53aab01aca8db729/ # Other
 │  │  │  ├── proc-macro2-57b7a19d526a43eb/ # Other
 │  │  │  ├── proc-macro2-631aca48eade5c45/ # Other
 │  │  │  ├── proc-macro2-6aaee8320f5eb30c/ # Other
 │  │  │  ├── proc-macro2-968717110cc73141/ # Other
 │  │  │  ├── proc-macro2-b7210b3e87279d82/ # Other
 │  │  │  ├── proc-macro2-c000e521c57088f1/ # Other
 │  │  │  ├── proc-macro2-diagnostics-424b445d75e7d9c4/ # Other
 │  │  │  ├── proc-macro2-diagnostics-74f3b6c0e8ef5eac/ # Other
 │  │  │  ├── proc-macro2-diagnostics-dcaa9a0155b5657b/ # Other
 │  │  │  ├── proc-macro2-diagnostics-f4bbf09723498c9d/ # Other
 │  │  │  ├── proc-macro2-e33d85e8a514aac3/ # Other
 │  │  │  ├── proc-macro2-e5e5052433162192/ # Other
 │  │  │  ├── proc-macro2-eae73f7019752462/ # Other
 │  │  │  ├── proc-macro2-fd4439c1391c1df5/ # Other
 │  │  │  ├── quote-1e530063a1f64fd0/ # Other
 │  │  │  ├── quote-5e2537f23c049761/ # Other
 │  │  │  ├── quote-5e3ec26c238cce27/ # Other
 │  │  │  ├── quote-7a4d6bc243d715ab/ # Other
 │  │  │  ├── quote-94e882aba9077123/ # Other
 │  │  │  ├── quote-fb5425fcfea6edd8/ # Other
 │  │  │  ├── quote-use-5ec0930e91bbe30e/ # Other
 │  │  │  ├── quote-use-d7854b198ff12e7a/ # Other
 │  │  │  ├── quote-use-macros-3b980f9e2f5a6682/ # Other
 │  │  │  ├── quote-use-macros-95db9fd8fd106532/ # Other
 │  │  │  ├── rand-178f11b086e662f0/ # Other
 │  │  │  ├── rand-4ccb40cfb62307b1/ # Other
 │  │  │  ├── rand-4fb31700415b00d9/ # Other
 │  │  │  ├── rand_chacha-045f9ccf6fda7ed7/ # Other
 │  │  │  ├── rand_chacha-2536855f2576ffef/ # Other
 │  │  │  ├── rand_core-8b2e112f593ecf27/ # Other
 │  │  │  ├── rand_core-e5b2c4a2527420db/ # Other
 │  │  │  ├── rand_pcg-b1542e49943bf419/ # Other
 │  │  │  ├── raw-window-handle-09ba32ac0fa85530/ # Other
 │  │  │  ├── regex-58a0e56a3f74a1e9/ # Other
 │  │  │  ├── regex-69c5edad41db4340/ # Other
 │  │  │  ├── regex-automata-2d4a5040a39f1329/ # Other
 │  │  │  ├── regex-automata-5858836c9d53a209/ # Other
 │  │  │  ├── regex-automata-6c56e300fc665edf/ # Other
 │  │  │  ├── regex-automata-90e0163ed6f94527/ # Other
 │  │  │  ├── regex-automata-b3135a381b13142c/ # Other
 │  │  │  ├── regex-d05050d7e3167dd1/ # Other
 │  │  │  ├── regex-f17a800c9f59b041/ # Other
 │  │  │  ├── regex-syntax-a07a0c358b3c4dbd/ # Other
 │  │  │  ├── regex-syntax-dc3e929c3e31c4da/ # Other
 │  │  │  ├── regex-syntax-ee1e14fca44c0b17/ # Other
 │  │  │  ├── reqwest-0a11b63f6a9c128c/ # Other
 │  │  │  ├── reqwest-2bd48be8114ddf55/ # Other
 │  │  │  ├── reqwest-8df0bb1cbf8dc3df/ # Other
 │  │  │  ├── rfd-899500aeca19d175/ # Other
 │  │  │  ├── rfd-abc5f3f0ac724c3b/ # Other
 │  │  │  ├── rfd-fb49998dcc36805a/ # Other
 │  │  │  ├── ring-116abca83523d089/ # Other
 │  │  │  ├── ring-486fbf2bb30088c3/ # Other
 │  │  │  ├── ring-4dcf1076e924578a/ # Other
 │  │  │  ├── ring-612246a69f22c61f/ # Other
 │  │  │  ├── ring-73c459dc7f060a1c/ # Other
 │  │  │  ├── ring-b6bb4e891e6f398d/ # Other
 │  │  │  ├── rstml-61d4466a16bb04e0/ # Other
 │  │  │  ├── rstml-f14ab914201742e0/ # Other
 │  │  │  ├── rustc-hash-b01ca692a41a5ac3/ # Other
 │  │  │  ├── rustc-hash-b39983ae2533e806/ # Other
 │  │  │  ├── rustc_version-e88169d1ef66dd87/ # Other
 │  │  │  ├── rustls-032730309f20ab80/ # Other
 │  │  │  ├── rustls-43f2af45c0bbec27/ # Other
 │  │  │  ├── rustls-ed19ac561ee1b997/ # Other
 │  │  │  ├── rustls-pemfile-02e1f245b5397614/ # Other
 │  │  │  ├── rustls-webpki-0ecb37abee4b72f2/ # Other
 │  │  │  ├── rustversion-139504f1e8c54e04/ # Other
 │  │  │  ├── rustversion-90a9414a6f185e53/ # Other
 │  │  │  ├── rustversion-d5ab1fab56c372b7/ # Other
 │  │  │  ├── ryu-6ac78a988ab9ad86/ # Other
 │  │  │  ├── ryu-9eaef482906fa0cf/ # Other
 │  │  │  ├── same-file-0ed655704eed581d/ # Other
 │  │  │  ├── same-file-99652511b315eafa/ # Other
 │  │  │  ├── same-file-d979a707f88fb31e/ # Other
 │  │  │  ├── same-file-fca5dc92c5cfb355/ # Other
 │  │  │  ├── schannel-e6968080c098a902/ # Other
 │  │  │  ├── scopeguard-0ddf3bfcacbcd354/ # Other
 │  │  │  ├── scopeguard-4a1404f096e49870/ # Other
 │  │  │  ├── scopeguard-4e06c38f24c23f21/ # Other
 │  │  │  ├── sct-c8192b0ccf19c63b/ # Other
 │  │  │  ├── selectors-04617866fec1b016/ # Other
 │  │  │  ├── selectors-31ff61d48655685f/ # Other
 │  │  │  ├── selectors-71a806b29c7e7948/ # Other
 │  │  │  ├── selectors-ba40892eb8b0d892/ # Other
 │  │  │  ├── self_cell-42600edba8ce07a8/ # Other
 │  │  │  ├── self_cell-ce282b763b6ed1ce/ # Other
 │  │  │  ├── semver-4f597c94ce4f7d59/ # Other
 │  │  │  ├── semver-704e96cc6d8be7b9/ # Other
 │  │  │  ├── semver-8ad401ef6d819402/ # Other
 │  │  │  ├── semver-b172dc7b0c76966d/ # Other
 │  │  │  ├── semver-c4e2705cf08470ba/ # Other
 │  │  │  ├── semver-c97dfeea256318a3/ # Other
 │  │  │  ├── serde-0468fd937fd5e9ba/ # Other
 │  │  │  ├── serde-167b1aa59de0fbdf/ # Other
 │  │  │  ├── serde-1f56252df68a0437/ # Other
 │  │  │  ├── serde-31740c4e77d37a9f/ # Other
 │  │  │  ├── serde-48803b33bdfc044a/ # Other
 │  │  │  ├── serde-533872101e102acb/ # Other
 │  │  │  ├── serde-5b6d9e0ec81445d5/ # Other
 │  │  │  ├── serde-670f440499e905ce/ # Other
 │  │  │  ├── serde-8767c121a099b839/ # Other
 │  │  │  ├── serde-a7cc8ec2b5a73461/ # Other
 │  │  │  ├── serde-cfbc768ae2461709/ # Other
 │  │  │  ├── serde-e3c3427e8dc1dfe2/ # Other
 │  │  │  ├── serde-f856b5be37e81ee8/ # Other
 │  │  │  ├── serde-wasm-bindgen-0755be63f6c9241e/ # Other
 │  │  │  ├── serde-wasm-bindgen-5b0011d2c42e40c2/ # Other
 │  │  │  ├── serde-wasm-bindgen-e12da64ff5b464cb/ # Other
 │  │  │  ├── serde-wasm-bindgen-f6d5856cb657b8a2/ # Other
 │  │  │  ├── serde_derive-0b6b8b3244311642/ # Other
 │  │  │  ├── serde_derive-47ca39cc2b9cc2a1/ # Other
 │  │  │  ├── serde_derive-5c15cfd401443024/ # Other
 │  │  │  ├── serde_derive-cc5e23863108000b/ # Other
 │  │  │  ├── serde_json-05aa2780d9f0df18/ # Other
 │  │  │  ├── serde_json-0beaee3737cdc7b2/ # Other
 │  │  │  ├── serde_json-47bf58c291c89eea/ # Other
 │  │  │  ├── serde_json-52e4fe8d81f4f2ed/ # Other
 │  │  │  ├── serde_json-5860b9e1019d3e2d/ # Other
 │  │  │  ├── serde_json-7466dbd8df3f5b25/ # Other
 │  │  │  ├── serde_json-b940e9690d189bbd/ # Other
 │  │  │  ├── serde_json-be641a788929b5a0/ # Other
 │  │  │  ├── serde_json-cea5d46714b5ad38/ # Other
 │  │  │  ├── serde_json-eab1c752ccf6340d/ # Other
 │  │  │  ├── serde_path_to_error-26e1b42c61277725/ # Other
 │  │  │  ├── serde_qs-4516876ccbba29ec/ # Other
 │  │  │  ├── serde_qs-b12985b9ecc83fda/ # Other
 │  │  │  ├── serde_repr-569119db4f4a063b/ # Other
 │  │  │  ├── serde_spanned-30ebfe06ce284436/ # Other
 │  │  │  ├── serde_test-11369039722918ed/ # Other
 │  │  │  ├── serde_test-7b71194b9619d136/ # Other
 │  │  │  ├── serde_urlencoded-2314ba4cd28de3dc/ # Other
 │  │  │  ├── serde_urlencoded-8595883566a873fe/ # Other
 │  │  │  ├── serde_urlencoded-ea144ce042aeeea0/ # Other
 │  │  │  ├── serde_with-d8679421cddbef08/ # Other
 │  │  │  ├── serde_with_macros-cde5fb94c52e7973/ # Other
 │  │  │  ├── serialize-to-javascript-21c1fee43bf5f3bf/ # Other
 │  │  │  ├── serialize-to-javascript-impl-136855055ad4a09a/ # Other
 │  │  │  ├── server_fn-b1e7f1a4cfe0392d/ # Other
 │  │  │  ├── server_fn-ce39ff96b213033a/ # Other
 │  │  │  ├── server_fn_macro-3bdfbd2f82b3f99a/ # Other
 │  │  │  ├── server_fn_macro-a189cce43ba54206/ # Other
 │  │  │  ├── server_fn_macro_default-0ec809b1217b7f68/ # Other
 │  │  │  ├── server_fn_macro_default-395affc2bb207456/ # Other
 │  │  │  ├── servo_arc-2a372508665aa6a9/ # Other
 │  │  │  ├── sha1-a0674114b8978380/ # Other
 │  │  │  ├── sha2-bef52d4820584750/ # Other
 │  │  │  ├── sha2-ea303993855079e1/ # Other
 │  │  │  ├── sharded-slab-246b66737d0748ef/ # Other
 │  │  │  ├── shlex-a918df6411f507de/ # Other
 │  │  │  ├── simd-adler32-fb0bc8735e3a2cbd/ # Other
 │  │  │  ├── simple_asn1-d3b99056f561b820/ # Other
 │  │  │  ├── siphasher-77157f26713148c2/ # Other
 │  │  │  ├── siphasher-fda3bf58e3c92d6d/ # Other
 │  │  │  ├── slab-0398d026498afef8/ # Other
 │  │  │  ├── slab-1caa8fbf185a3a31/ # Other
 │  │  │  ├── slab-72f34d3e45a32922/ # Other
 │  │  │  ├── slab-fde0d11161d72c20/ # Other
 │  │  │  ├── slotmap-1344db00c2915440/ # Other
 │  │  │  ├── slotmap-14d226e7b57b4c7c/ # Other
 │  │  │  ├── slotmap-e1c1f9518a122b13/ # Other
 │  │  │  ├── slotmap-f79f1c5ea9869ee9/ # Other
 │  │  │  ├── smallvec-23bdbb9eee51fef3/ # Other
 │  │  │  ├── smallvec-80f5201b0ce60e09/ # Other
 │  │  │  ├── smallvec-a63e90090a854e1e/ # Other
 │  │  │  ├── socket2-9197175e17c66b74/ # Other
 │  │  │  ├── socket2-dc94843b43bef8e7/ # Other
 │  │  │  ├── socket2-ea003b1fc034b781/ # Other
 │  │  │  ├── spin-37e19d390bd0f360/ # Other
 │  │  │  ├── spin-a1d705d907adac25/ # Other
 │  │  │  ├── sqlformat-ba9bfef8131f18f5/ # Other
 │  │  │  ├── sqlx-8af7e13c8660636a/ # Other
 │  │  │  ├── sqlx-core-904cd891380d4cbf/ # Other
 │  │  │  ├── sqlx-core-eb89f501b27282e6/ # Other
 │  │  │  ├── sqlx-macros-core-bb6c4cf5159867ae/ # Other
 │  │  │  ├── sqlx-macros-ee93fff252e39bd1/ # Other
 │  │  │  ├── sqlx-sqlite-4d9961cb631144f7/ # Other
 │  │  │  ├── sqlx-sqlite-df0d63e131a7eea9/ # Other
 │  │  │  ├── stable_deref_trait-9c3667d1124708cb/ # Other
 │  │  │  ├── stable_deref_trait-b3a89a2ed4349c7a/ # Other
 │  │  │  ├── stable_deref_trait-fd6a8db73e3da569/ # Other
 │  │  │  ├── state-ff0f3f271f6f4d69/ # Other
 │  │  │  ├── string_cache-1ff4920e2fa25379/ # Other
 │  │  │  ├── string_cache_codegen-2ffce7f432f1d688/ # Other
 │  │  │  ├── strsim-9b129a8a078e578d/ # Other
 │  │  │  ├── subtle-7a62713adcbc7e72/ # Other
 │  │  │  ├── syn-0c2f48d655f2cb95/ # Other
 │  │  │  ├── syn-2e77c2f2daf430ef/ # Other
 │  │  │  ├── syn-370473c0acc35b36/ # Other
 │  │  │  ├── syn-3dbeed824ac2dd61/ # Other
 │  │  │  ├── syn-6e39e98972bfc421/ # Other
 │  │  │  ├── syn-73f31af163b26b5a/ # Other
 │  │  │  ├── syn-76b300b3f9d51e1e/ # Other
 │  │  │  ├── syn-af73a8c2304a83c8/ # Other
 │  │  │  ├── syn-e22d705e2ebbff72/ # Other
 │  │  │  ├── syn_derive-3a6b7784ffca3495/ # Other
 │  │  │  ├── syn_derive-453f05ab82e757b9/ # Other
 │  │  │  ├── sync_wrapper-0c8fe211af9f6926/ # Other
 │  │  │  ├── sync_wrapper-ebb80cc489b05712/ # Other
 │  │  │  ├── synstructure-6863203cb8af616a/ # Other
 │  │  │  ├── synstructure-6b4e99f95083ac2f/ # Other
 │  │  │  ├── synstructure-99a68162528bdd62/ # Other
 │  │  │  ├── tao-4e4a3e65e1c54c50/ # Other
 │  │  │  ├── tao-d93ef312fa230914/ # Other
 │  │  │  ├── tao-fd0765ee8abe7eb5/ # Other
 │  │  │  ├── tar-f639cdb04416da85/ # Other
 │  │  │  ├── tauri-33be9bb694e6dd84/ # Other
 │  │  │  ├── tauri-6d605ca8adc54ff8/ # Other
 │  │  │  ├── tauri-c212520e9e56c3ae/ # Other
 │  │  │  ├── tauri-codegen-222a42f26bc392fe/ # Other
 │  │  │  ├── tauri-macros-c06773f2241d9966/ # Other
 │  │  │  ├── tauri-runtime-01c29ba968439614/ # Other
 │  │  │  ├── tauri-runtime-3655de93e56d586d/ # Other
 │  │  │  ├── tauri-runtime-c860fe0fed7a3d22/ # Other
 │  │  │  ├── tauri-runtime-wry-59ed0ba4b63cf749/ # Other
 │  │  │  ├── tauri-runtime-wry-6d4af70234f4edc6/ # Other
 │  │  │  ├── tauri-runtime-wry-f6fbd9c2bdfd57d1/ # Other
 │  │  │  ├── tauri-utils-850ec84ca3cd3bf9/ # Other
 │  │  │  ├── tauri-utils-8c56bf09764b2b9b/ # Other
 │  │  │  ├── tauri-winres-82f116dc5f270d24/ # Other
 │  │  │  ├── tempfile-5c1cabad4d974680/ # Other
 │  │  │  ├── tempfile-879a78ce8cbb4360/ # Other
 │  │  │  ├── tendril-a62f39c6a0830c7b/ # Other
 │  │  │  ├── thin-slice-389a0e92a0fd4a34/ # Other
 │  │  │  ├── thiserror-31647a66f939fece/ # Other
 │  │  │  ├── thiserror-5370e29d70fcf0bc/ # Other
 │  │  │  ├── thiserror-537b29e143a893b3/ # Other
 │  │  │  ├── thiserror-5a17a0255dfc7fbf/ # Other
 │  │  │  ├── thiserror-6652d28f992b0e2d/ # Other
 │  │  │  ├── thiserror-83c827a4f6cce979/ # Other
 │  │  │  ├── thiserror-8652639a0cc6527e/ # Other
 │  │  │  ├── thiserror-a3add442fd4f9e04/ # Other
 │  │  │  ├── thiserror-ec886bbb586d94a7/ # Other
 │  │  │  ├── thiserror-impl-ba8e44e614c3ca47/ # Other
 │  │  │  ├── thiserror-impl-d36af86107c4d025/ # Other
 │  │  │  ├── thiserror-impl-e15f8928fe713498/ # Other
 │  │  │  ├── thiserror-impl-fcd90585e61c2949/ # Other
 │  │  │  ├── thread_local-76db8d85b7d436aa/ # Other
 │  │  │  ├── time-533edd9529cdd213/ # Other
 │  │  │  ├── time-core-defdcd232061ec7a/ # Other
 │  │  │  ├── time-macros-c0d5df18541a71e1/ # Other
 │  │  │  ├── tinystr-29aea4be1ebf6f76/ # Other
 │  │  │  ├── tinystr-c8b3483d889426df/ # Other
 │  │  │  ├── tinystr-f85a49bcfd89d7f8/ # Other
 │  │  │  ├── tokio-3afdb0ffe8b0a7f2/ # Other
 │  │  │  ├── tokio-4f248e107ba98576/ # Other
 │  │  │  ├── tokio-9a3fc81a15e75b7d/ # Other
 │  │  │  ├── tokio-b83370a558cbcebf/ # Other
 │  │  │  ├── tokio-macros-342f7d1c003de12e/ # Other
 │  │  │  ├── tokio-macros-516946824e2ab316/ # Other
 │  │  │  ├── tokio-macros-70a3a3df619b2cf6/ # Other
 │  │  │  ├── tokio-native-tls-db289786cf566cc4/ # Other
 │  │  │  ├── tokio-stream-63f0012cb9c4c568/ # Other
 │  │  │  ├── tokio-stream-870c4c98e99240b4/ # Other
 │  │  │  ├── tokio-util-3c4fec01908767ea/ # Other
 │  │  │  ├── tokio-util-afb82cc70712256a/ # Other
 │  │  │  ├── tokio-util-c83088f349e3e62d/ # Other
 │  │  │  ├── toml-31e1b314345a7cab/ # Other
 │  │  │  ├── toml-6b5c3decad55024e/ # Other
 │  │  │  ├── toml-a1b3b9baea6723c4/ # Other
 │  │  │  ├── toml-bea85f66ff45f2a7/ # Other
 │  │  │  ├── toml_datetime-7ca85b9526bd94e2/ # Other
 │  │  │  ├── toml_edit-1b52e60d7e662fad/ # Other
 │  │  │  ├── toml_edit-952276aa76529073/ # Other
 │  │  │  ├── tower-2db87431e8f263a6/ # Other
 │  │  │  ├── tower-http-03c2ae372b0fd587/ # Other
 │  │  │  ├── tower-layer-e54a78936e51308c/ # Other
 │  │  │  ├── tower-service-55549b0459386dc4/ # Other
 │  │  │  ├── tower-service-85c893f18d7f8b9f/ # Other
 │  │  │  ├── tracing-242c740104dda2b0/ # Other
 │  │  │  ├── tracing-attributes-85c31237866d18f2/ # Other
 │  │  │  ├── tracing-attributes-ee535dd7a223348a/ # Other
 │  │  │  ├── tracing-attributes-f7c39ce0388ccb27/ # Other
 │  │  │  ├── tracing-b7edb7f9bb42301f/ # Other
 │  │  │  ├── tracing-bd0aeb61dff9692d/ # Other
 │  │  │  ├── tracing-be271e4b8e3e8ac0/ # Other
 │  │  │  ├── tracing-bf0403ff45de094c/ # Other
 │  │  │  ├── tracing-core-0bc12b3517250cf2/ # Other
 │  │  │  ├── tracing-core-6f68029718df92f8/ # Other
 │  │  │  ├── tracing-core-ac58a466b798ad44/ # Other
 │  │  │  ├── tracing-core-f56912c40f75e7ef/ # Other
 │  │  │  ├── tracing-core-fc56c0b93b8f8ad6/ # Other
 │  │  │  ├── tracing-log-5631a2963aa15363/ # Other
 │  │  │  ├── tracing-subscriber-2c9dfd7ab20e3e8c/ # Other
 │  │  │  ├── try-lock-1d7644d8c8c15ffe/ # Other
 │  │  │  ├── try-lock-b0e4eaf6395ecfb6/ # Other
 │  │  │  ├── typenum-3be11317a6652575/ # Other
 │  │  │  ├── typenum-c9d9165c9c15b87c/ # Other
 │  │  │  ├── typenum-f4e2056ab3ddd92c/ # Other
 │  │  │  ├── unicase-cf8ab1c8042ff506/ # Other
 │  │  │  ├── unicode-ident-05b1ed5289651a81/ # Other
 │  │  │  ├── unicode-ident-821f2f272d853d79/ # Other
 │  │  │  ├── unicode-ident-e9d3b26fe86656b0/ # Other
 │  │  │  ├── unicode-segmentation-7792071ebc7b96be/ # Other
 │  │  │  ├── unicode-segmentation-b6580527f283b27a/ # Other
 │  │  │  ├── unicode-xid-277d5b84f1fae7f1/ # Other
 │  │  │  ├── unicode_categories-50444fdeeeed1a7b/ # Other
 │  │  │  ├── untrusted-1def58ec6b7b2f4e/ # Other
 │  │  │  ├── untrusted-c25bf2311f1ff702/ # Other
 │  │  │  ├── url-36540d44782642fd/ # Other
 │  │  │  ├── url-4c060710b751fd81/ # Other
 │  │  │  ├── url-69e52efd6a5a941c/ # Other
 │  │  │  ├── url-fc9fb3cd1878ce8a/ # Other
 │  │  │  ├── urlencoding-cd58a3e4538fc6fe/ # Other
 │  │  │  ├── utf-8-c6f146d0dba3273f/ # Other
 │  │  │  ├── utf16_iter-27e504796cb8d100/ # Other
 │  │  │  ├── utf16_iter-fa08d0f24d3b01a3/ # Other
 │  │  │  ├── utf8-width-082ba1de68542210/ # Other
 │  │  │  ├── utf8-width-b28cff048123ab39/ # Other
 │  │  │  ├── utf8-width-d6eb82e2b39b1174/ # Other
 │  │  │  ├── utf8_iter-470e50d0e67000d5/ # Other
 │  │  │  ├── utf8_iter-ad445a2c233037c3/ # Other
 │  │  │  ├── uuid-296daba6f37f6ced/ # Other
 │  │  │  ├── uuid-817a1c7bc948500b/ # Other
 │  │  │  ├── uuid-90275ae07e654ed0/ # Other
 │  │  │  ├── uuid-b1dabc10874d36ed/ # Other
 │  │  │  ├── vcpkg-066dd131759a4251/ # Other
 │  │  │  ├── version_check-cb5a7676e4932a43/ # Other
 │  │  │  ├── vswhom-9cfdcfe69b8410e2/ # Other
 │  │  │  ├── vswhom-sys-60a7ade249003848/ # Other
 │  │  │  ├── vswhom-sys-9babd224d815be5b/ # Other
 │  │  │  ├── vswhom-sys-f877ceac7c3daf06/ # Other
 │  │  │  ├── walkdir-292467393105b186/ # Other
 │  │  │  ├── walkdir-2c2d7437cfb1e3a8/ # Other
 │  │  │  ├── walkdir-39a32b52299c2287/ # Other
 │  │  │  ├── walkdir-6b8c66830f52d36f/ # Other
 │  │  │  ├── want-30f0edabf195909e/ # Other
 │  │  │  ├── want-67e7bc29f1287585/ # Other
 │  │  │  ├── wasm-bindgen-52087da43f492494/ # Other
 │  │  │  ├── wasm-bindgen-95f1388d76308058/ # Other
 │  │  │  ├── wasm-bindgen-9677b9b2d9850b44/ # Other
 │  │  │  ├── wasm-bindgen-9f84acbe7c51d350/ # Other
 │  │  │  ├── wasm-bindgen-backend-74e85bf4bcc6254e/ # Other
 │  │  │  ├── wasm-bindgen-backend-75b1eff8c13557e4/ # Other
 │  │  │  ├── wasm-bindgen-futures-8e2d7ccaba3e70d0/ # Other
 │  │  │  ├── wasm-bindgen-futures-dfeb5c8bed98a8b3/ # Other
 │  │  │  ├── wasm-bindgen-macro-a5e0b872209fb287/ # Other
 │  │  │  ├── wasm-bindgen-macro-e8683383e2f70fe5/ # Other
 │  │  │  ├── wasm-bindgen-macro-support-aaed6601df312506/ # Other
 │  │  │  ├── wasm-bindgen-macro-support-b49fa3e5b4232e55/ # Other
 │  │  │  ├── wasm-bindgen-shared-2312126c3a6d93f5/ # Other
 │  │  │  ├── wasm-bindgen-shared-60f5bbc1ff07995c/ # Other
 │  │  │  ├── wasm-bindgen-shared-6747609f6370f3c7/ # Other
 │  │  │  ├── wasm-bindgen-shared-e1224233b6671c09/ # Other
 │  │  │  ├── web-sys-21e13f3f85c457a7/ # Other
 │  │  │  ├── web-sys-533db6ff26bf1ae2/ # Other
 │  │  │  ├── webpki-roots-9b85f857f68c05df/ # Other
 │  │  │  ├── webview2-com-a192c6abd084fd63/ # Other
 │  │  │  ├── webview2-com-macros-c25839bad825e89d/ # Other
 │  │  │  ├── webview2-com-sys-4f8d867a37a3ed79/ # Other
 │  │  │  ├── webview2-com-sys-7521e9cdb9ec512e/ # Other
 │  │  │  ├── webview2-com-sys-a6b585fe977c291a/ # Other
 │  │  │  ├── winapi-3ae5a410c16dc6e0/ # Other
 │  │  │  ├── winapi-7887ff730b52d998/ # Other
 │  │  │  ├── winapi-843170f6f3cc14df/ # Other
 │  │  │  ├── winapi-d63427466bad987f/ # Other
 │  │  │  ├── winapi-e6083cfde85f86fa/ # Other
 │  │  │  ├── winapi-efa1be7238608458/ # Other
 │  │  │  ├── winapi-util-34414de1ec6192dc/ # Other
 │  │  │  ├── winapi-util-3f0fd35d4ecac6e5/ # Other
 │  │  │  ├── winapi-util-6129751e67f39b9b/ # Other
 │  │  │  ├── winapi-util-68a7505b8b632c0c/ # Other
 │  │  │  ├── windows-59f865d22d11220f/ # Other
 │  │  │  ├── windows-b32c51b6e76dd7d4/ # Other
 │  │  │  ├── windows-bindgen-8c21400adf5bdf01/ # Other
 │  │  │  ├── windows-implement-0a1689933e276e4b/ # Other
 │  │  │  ├── windows-link-7d8b2e9c4c517ee5/ # Other
 │  │  │  ├── windows-metadata-00081d6a7adb377d/ # Other
 │  │  │  ├── windows-sys-0a592bec28d709a5/ # Other
 │  │  │  ├── windows-sys-21246cd14f32ba18/ # Other
 │  │  │  ├── windows-sys-2e3ee3fb155c4693/ # Other
 │  │  │  ├── windows-sys-409b77213d0d0c25/ # Other
 │  │  │  ├── windows-sys-4d624ab5cb399701/ # Other
 │  │  │  ├── windows-sys-7c2716a9b133ff32/ # Other
 │  │  │  ├── windows-sys-96ca9f7c31ec0bfa/ # Other
 │  │  │  ├── windows-sys-ca3352e5610e7c91/ # Other
 │  │  │  ├── windows-sys-cbcda74051c6d1fd/ # Other
 │  │  │  ├── windows-targets-215bdda23ea50391/ # Other
 │  │  │  ├── windows-targets-4247ab0bb8beda0d/ # Other
 │  │  │  ├── windows-targets-83e22dcff0a1c380/ # Other
 │  │  │  ├── windows-targets-dd8cca008988cf75/ # Other
 │  │  │  ├── windows-targets-f658f535569adcb5/ # Other
 │  │  │  ├── windows-tokens-e906d00c6ec0a1e8/ # Other
 │  │  │  ├── windows-version-521aa537262db6e3/ # Other
 │  │  │  ├── windows_x86_64_msvc-0b81e60db8677a5a/ # Other
 │  │  │  ├── windows_x86_64_msvc-0b8420bd98bfbf76/ # Other
 │  │  │  ├── windows_x86_64_msvc-2df3a76ecc6bee7b/ # Other
 │  │  │  ├── windows_x86_64_msvc-4942fa3d6acbcf58/ # Other
 │  │  │  ├── windows_x86_64_msvc-506f3ae964f5d614/ # Other
 │  │  │  ├── windows_x86_64_msvc-5d86f23de07b454a/ # Other
 │  │  │  ├── windows_x86_64_msvc-668dd25a1388a7b2/ # Other
 │  │  │  ├── windows_x86_64_msvc-6df32fa3524ce425/ # Other
 │  │  │  ├── windows_x86_64_msvc-80d91f243cf132a3/ # Other
 │  │  │  ├── windows_x86_64_msvc-907d2824ecce8709/ # Other
 │  │  │  ├── windows_x86_64_msvc-9a75be1557ec4f53/ # Other
 │  │  │  ├── windows_x86_64_msvc-cfbc318abc5c20e9/ # Other
 │  │  │  ├── windows_x86_64_msvc-d03921a068d133b3/ # Other
 │  │  │  ├── windows_x86_64_msvc-d34505db9e8c60be/ # Other
 │  │  │  ├── windows_x86_64_msvc-e05cdbc621eda289/ # Other
 │  │  │  ├── windows_x86_64_msvc-ebd3ac8ade334356/ # Other
 │  │  │  ├── windows_x86_64_msvc-f10211265d8162be/ # Other
 │  │  │  ├── windows_x86_64_msvc-fb5a8d7c417fe5b3/ # Other
 │  │  │  ├── winnow-754f402709bd3f98/ # Other
 │  │  │  ├── winnow-d761b9f3cd64d6f6/ # Other
 │  │  │  ├── winreg-483446f7970b1780/ # Other
 │  │  │  ├── winreg-9f064dc4d8aa4316/ # Other
 │  │  │  ├── winreg-bcc851e06f4b9135/ # Other
 │  │  │  ├── write16-0f669802e9093f18/ # Other
 │  │  │  ├── write16-bbe237623a800d2c/ # Other
 │  │  │  ├── writeable-484bb341793a49d8/ # Other
 │  │  │  ├── writeable-8437087a151df4e0/ # Other
 │  │  │  ├── wry-5d8ff5432334d90e/ # Other
 │  │  │  ├── wry-b7e125029ce29a53/ # Other
 │  │  │  ├── wry-c001605d0ad4db70/ # Other
 │  │  │  ├── xxhash-rust-243c6b9e42e92b14/ # Other
 │  │  │  ├── xxhash-rust-3666208c04d2299e/ # Other
 │  │  │  ├── xxhash-rust-8493ff7c83af91b6/ # Other
 │  │  │  ├── yansi-36f15a3156c9d803/ # Other
 │  │  │  ├── yoke-54bc9dccaaabfd56/ # Other
 │  │  │  ├── yoke-ccb76b75b4106e92/ # Other
 │  │  │  ├── yoke-derive-44e57d5d5fab2756/ # Other
 │  │  │  ├── yoke-derive-9d64d9c34f94d7b6/ # Other
 │  │  │  ├── yoke-derive-c198dcfc7a29d48c/ # Other
 │  │  │  ├── yoke-f6bd1cc17699bbe5/ # Other
 │  │  │  ├── zerocopy-153b9f2a2e913403/ # Other
 │  │  │  ├── zerocopy-3c9564961073d2cf/ # Other
 │  │  │  ├── zerocopy-4fc916ac2a54efcb/ # Other
 │  │  │  ├── zerocopy-dc81a530e978587f/ # Other
 │  │  │  ├── zerofrom-5eeb07004b908855/ # Other
 │  │  │  ├── zerofrom-770712b6dd29a699/ # Other
 │  │  │  ├── zerofrom-de2ec600996a11b6/ # Other
 │  │  │  ├── zerofrom-derive-9a069b37fd051c57/ # Other
 │  │  │  ├── zerofrom-derive-ce559d753b3fa737/ # Other
 │  │  │  ├── zerofrom-derive-f6e666ee1f73184b/ # Other
 │  │  │  ├── zeroize-c08345c222b1583f/ # Other
 │  │  │  ├── zerovec-406c5db35c33efbd/ # Other
 │  │  │  ├── zerovec-5090edc86d9eaa55/ # Other
 │  │  │  ├── zerovec-67773bb0edc44f2f/ # Other
 │  │  │  ├── zerovec-derive-4737107d40b22ad9/ # Other
 │  │  │  ├── zerovec-derive-843eb5fecdf0be0b/ # Other
 │  │  │  └── zerovec-derive-d9eb2a81f95cebab/ # Other
 │  │  ├── deps/ # Other
 │  │  ├── examples/ # Other
 │  │  ├── incremental/ # Other
 │  │  │  ├── lms_shared-3215uk5tyelme/ # Other
 │  │  │  │  └── s-h63dvfkfdz-0cwuxup-aysg6p2qva3774s3lwd06xit9/ # Other
 │  │  │  ├── lms_ui-0ubxjm6fo5i2c/ # Other
 │  │  │  │  └── s-h635hjww7i-07na605-working/ # Other
 │  │  │  ├── lms_ui-102apfnwk9l8p/ # Other
 │  │  │  │  └── s-h63nmtr29p-1yeqa1f-working/ # Other
 │  │  │  ├── lms_ui-10mu1180ygwdx/ # Other
 │  │  │  │  └── s-h6357bps0h-0e2htxk-working/ # Other
 ├── tests/ # Other
 │  └── integration/ # Other
 ├── tools/ # Other
 │  └── __pycache__/ # Other
```


## Implementation Details

### Models (89% Complete)

| Model | File | Completeness |
|-------|------|-------------|
| Assignment | shared/models/course.rs | 50%  |
| Assignment | src-tauri/src/models/course.rs | 50%  |
| AuthResponse | shared/models/user.rs | 39% ⚠️ Low |
| AuthResponse | shared/src/models/user.rs | 39% ⚠️ Low |
| Category | src-tauri/src/models/category.rs | 60%  |
| Course | shared/models/course.rs | 55%  |
| Course | shared/src/models/course.rs | 47% ⚠️ Low |
| Course | src-tauri/src/models/course.rs | 50%  |
| CourseStatus | shared/models/course.rs | 32% ⚠️ Low |
| CourseStatus | src-tauri/src/models/course.rs | 32% ⚠️ Low |
| Enrollment | shared/models/course.rs | 47% ⚠️ Low |
| EnrollmentRole | shared/models/course.rs | 32% ⚠️ Low |
| ForumCategory | shared/models/forum.rs | 50%  |
| ForumCategory | shared/src/models/forum.rs | 43% ⚠️ Low |
| ForumPost | shared/models/forum.rs | 50%  |
| ForumPost | shared/src/models/forum.rs | 45% ⚠️ Low |
| ForumTopic | shared/models/forum.rs | 50%  |
| ForumTopic | shared/src/models/forum.rs | 45% ⚠️ Low |
| ForumTrustLevel | shared/models/forum.rs | 47% ⚠️ Low |
| ForumUserPreferences | shared/models/forum.rs | 49% ⚠️ Low |
| LoginRequest | shared/models/user.rs | 39% ⚠️ Low |
| LoginRequest | shared/src/models/user.rs | 39% ⚠️ Low |
| Module | shared/models/course.rs | 50%  |
| Module | src-tauri/src/models/course.rs | 50%  |
| Post | src-tauri/src/models/post.rs | 60%  |
| RegisterRequest | shared/models/user.rs | 41% ⚠️ Low |
| RegisterRequest | shared/src/models/user.rs | 43% ⚠️ Low |
| Submission | shared/models/course.rs | 49% ⚠️ Low |
| Submission | src-tauri/src/models/course.rs | 49% ⚠️ Low |
| Tag | src-tauri/src/models/tag.rs | 57%  |
| Topic | src-tauri/src/models/topic.rs | 60%  |
| User | shared/models/user.rs | 60%  |
| User | shared/src/models/user.rs | 45% ⚠️ Low |
| User | src-tauri/src/models/user.rs | 60%  |
| UserProfile | shared/models/user.rs | 41% ⚠️ Low |
| UserRole | shared/models/user.rs | 45% ⚠️ Low |
| UserRole | src-tauri/src/models/user.rs | 32% ⚠️ Low |

### API Endpoints (0% Complete)

| Handler | File | Route | Completeness | Feature Area |
|---------|------|-------|-------------|--------------|
| get(get_current_user | src-tauri/src/main.rs | - | 20% ⚠️ Low | auth |
| post(login_user | src-tauri/src/main.rs | - | 20% ⚠️ Low | auth |
| post(register_user | src-tauri/src/main.rs | - | 20% ⚠️ Low | auth |
| put(update_user_profile | src-tauri/src/main.rs | - | 20% ⚠️ Low | auth |
| create_category | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| create_post | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| create_topic | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| delete_category | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| delete_post | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| delete_topic | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| delete(delete_category | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| delete(delete_post | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| delete(delete_topic | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get_assignment_topic | src-tauri/src/api/mod.rs | - | 20% ⚠️ Low | forum |
| get_categories | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get_category | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get_course_category | src-tauri/src/api/mod.rs | - | 20% ⚠️ Low | forum |
| get_course_forum_activity | src-tauri/src/api/mod.rs | - | 20% ⚠️ Low | forum |
| get_module_topic | src-tauri/src/api/mod.rs | - | 20% ⚠️ Low | forum |
| get_post | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get_posts_for_topic | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get_recent_posts | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get_recent_topics | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get_topic | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get_topics | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get_topics_by_category | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(get_categories | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(get_categories_by_course | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(get_category | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(get_category | src-tauri/src/main.rs | - | 20% ⚠️ Low | forum |
| get(get_forum_stats | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(get_post | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(get_post | src-tauri/src/main.rs | - | 20% ⚠️ Low | forum |
| get(get_posts_by_topic | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(get_recent_topics | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(get_tags | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(get_topic | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(get_topic | src-tauri/src/main.rs | - | 20% ⚠️ Low | forum |
| get(get_topics | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(get_topics_by_category | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(get_topics_by_tag | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(get_updated_categories | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(get_updated_posts | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(get_updated_topics | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(list_topic_posts | src-tauri/src/main.rs | - | 20% ⚠️ Low | forum |
| get(list_topics | src-tauri/src/main.rs | - | 20% ⚠️ Low | forum |
| get(search_forum | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| like_post | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| mark_as_solution | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| post(create_category | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| post(create_category | src-tauri/src/main.rs | - | 20% ⚠️ Low | forum |
| post(create_post | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| post(create_post | src-tauri/src/main.rs | - | 20% ⚠️ Low | forum |
| post(create_topic | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| post(create_topic | src-tauri/src/main.rs | - | 20% ⚠️ Low | forum |
| post(like_post | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| put(update_category | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| put(update_category | src-tauri/src/main.rs | - | 20% ⚠️ Low | forum |
| put(update_post | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| put(update_topic | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| unlike_post | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| update_category | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| update_post | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| update_topic | src-tauri/src/api/forum.rs | - | 20% ⚠️ Low | forum |
| get(health_check | src-tauri/src/api/mod.rs | - | 20% ⚠️ Low | other |
| get(list_categories | src-tauri/src/main.rs | - | 20% ⚠️ Low | other |
| get(root | src-tauri/src/main.rs | - | 20% ⚠️ Low | other |

### UI Components (90% Complete)

| Component | File | Completeness |
|-----------|------|-------------|
| ActivityItem | src/components/shared/activity_stream.rs | 35% ⚠️ Low |
| ActivityLog | src/components/forum/admin/activity_log.rs | 40% ⚠️ Low |
| ActivityStream | src/components/shared/activity_stream.rs | 40% ⚠️ Low |
| AdminCategories | src/components/admin/categories.rs | 40% ⚠️ Low |
| AdminDashboard | src/components/admin/dashboard.rs | 40% ⚠️ Low |
| AdminDashboard | src/components/forum/admin/dashboard.rs | 40% ⚠️ Low |
| AdminLayout | src/components/admin/layout.rs | 35% ⚠️ Low |
| AdminLayout | src/components/forum/admin/admin_layout.rs | 35% ⚠️ Low |
| AdminSidebar | src/components/admin/layout.rs | 35% ⚠️ Low |
| AdminUsers | src/components/admin/users.rs | 25% ⚠️ Low |
| AllNotifications | src/components/forum/all_notifications.rs | 40% ⚠️ Low |
| App | src/app.rs | 25% ⚠️ Low |
| AppLayout | src/components/layout/app_layout.rs | 35% ⚠️ Low |
| AppLayout | src/components/layout.rs | 35% ⚠️ Low |
| AssignmentDetail | src/components/lms/assignments.rs | 40% ⚠️ Low |
| AssignmentDetail | src/pages/assignment_detail.rs | 35% ⚠️ Low |
| AssignmentDiscussion | src/components/assignment_discussion.rs | 40% ⚠️ Low |
| AssignmentDiscussions | src/components/assignment_discussions.rs | 40% ⚠️ Low |
| AssignmentForm | src/components/lms/assignments.rs | 40% ⚠️ Low |
| AssignmentsList | src/components/lms/assignments.rs | 40% ⚠️ Low |
| AuthProvider | src/components/auth.rs | 45% ⚠️ Low |
| BookmarkButton | src/components/forum/topics/bookmark_button.rs | 40% ⚠️ Low |
| CategoriesList | src/components/forum/categories_list.rs | 40% ⚠️ Low |
| CategoryDetail | src/components/categories.rs | 35% ⚠️ Low |
| CategoryDetail | src/components/forum/category_detail.rs | 40% ⚠️ Low |
| CategoryForm | src/components/categories.rs | 50%  |
| CategoryForm | src/components/forum/category_form.rs | 40% ⚠️ Low |
| CategoryManagement | src/components/forum/admin/category_management.rs | 40% ⚠️ Low |
| CourseCategoryLinker | src/components/shared/course_forum_linker.rs | 40% ⚠️ Low |
| CourseDetail | src/components/courses/course_detail.rs | 40% ⚠️ Low |
| CourseDetail | src/components/lms/courses.rs | 40% ⚠️ Low |
| CourseDetail | src/pages/course_detail.rs | 35% ⚠️ Low |
| CourseForm | src/components/lms/courses.rs | 20% ⚠️ Low |
| CourseForum | src/pages/course_forum.rs | 25% ⚠️ Low |
| CourseForumActivity | src/components/course_forum_activity.rs | 40% ⚠️ Low |
| CourseList | src/components/courses/course_list.rs | 45% ⚠️ Low |
| CoursesList | src/components/lms/courses.rs | 40% ⚠️ Low |
| Dashboard | src/components/dashboard.rs | 40% ⚠️ Low |
| Dashboard | src/features/dashboard/dashboard_view.rs | 35% ⚠️ Low |
| ErrorDisplay | src/components/shared/error_display.rs | 35% ⚠️ Low |
| Footer | src/components/layout/footer.rs | 35% ⚠️ Low |
| Footer | src/components/layout.rs | 35% ⚠️ Low |
| ForumActivityWidget | src/components/forum_activity_widget.rs | 40% ⚠️ Low |
| ForumCategories | src/components/categories.rs | 40% ⚠️ Low |
| ForumCategories | src/components/forum/categories.rs | 40% ⚠️ Low |
| ForumSearch | src/components/forum/forum_search.rs | 20% ⚠️ Low |
| ForumSettings | src/components/forum/admin/forum_settings.rs | 40% ⚠️ Low |
| ForumThreads | src/components/forum/forum_threads.rs | 40% ⚠️ Low |
| ForumThreads | src/components/forum/threads.rs | 40% ⚠️ Low |
| GroupManagement | src/components/forum/group_management.rs | 40% ⚠️ Low |
| Header | src/components/layout/header.rs | 35% ⚠️ Low |
| Header | src/components/layout.rs | 40% ⚠️ Low |
| Home | src/app.rs | 35% ⚠️ Low |
| Home | src/components/home.rs | 45% ⚠️ Low |
| ImportExport | src/components/forum/admin/import_export.rs | 40% ⚠️ Low |
| IntegrationDashboard | src/components/shared/integration_dashboard.rs | 35% ⚠️ Low |
| Layout | src/components/layout.rs | 35% ⚠️ Low |
| Login | src/components/auth/login.rs | 45% ⚠️ Low |
| Login | src/components/auth.rs | 50%  |
| LoginForm | src/components/auth.rs | 40% ⚠️ Low |
| ModerationQueue | src/components/forum/admin/moderation_queue.rs | 25% ⚠️ Low |
| ModuleDetail | src/components/lms/modules.rs | 40% ⚠️ Low |
| ModuleDetail | src/pages/module_detail.rs | 35% ⚠️ Low |
| ModuleDiscussion | src/components/module_discussion.rs | 40% ⚠️ Low |
| ModuleDiscussions | src/components/module_discussions.rs | 40% ⚠️ Low |
| ModuleForm | src/components/lms/modules.rs | 40% ⚠️ Low |
| ModuleItemForm | src/components/lms/modules.rs | 40% ⚠️ Low |
| ModuleItemForm | src/components/lms/module_items.rs | 40% ⚠️ Low |
| ModulesList | src/components/lms/modules.rs | 40% ⚠️ Low |
| NotFound | src/app.rs | 35% ⚠️ Low |
| NotificationCenter | src/components/forum/notifications/notification_center.rs | 40% ⚠️ Low |
| NotificationDropdown | src/components/forum/notifications/notification_dropdown.rs | 40% ⚠️ Low |
| NotificationIndicator | src/components/forum/notification_indicator.rs | 40% ⚠️ Low |
| NotificationSettings | src/components/admin/notification_settings.rs | 40% ⚠️ Low |
| NotificationsList | src/components/forum/notifications/notifications_list.rs | 40% ⚠️ Low |
| NotificationsPage | src/components/forum/notifications/notifications_page.rs | 40% ⚠️ Low |
| OfflineIndicator | src/app.rs | 35% ⚠️ Low |
| OfflineIndicator | src/components/shared/offline_indicator.rs | 35% ⚠️ Low |
| Pagination | src/components/common/pagination.rs | 20% ⚠️ Low |
| ProfileEdit | src/components/forum/profile_edit.rs | 40% ⚠️ Low |
| Register | src/components/auth/register.rs | 45% ⚠️ Low |
| Register | src/components/auth.rs | 50%  |
| RegisterForm | src/components/auth.rs | 40% ⚠️ Low |
| ReportedContent | src/components/forum/admin/reported_content.rs | 40% ⚠️ Low |
| RichEditor | src/components/forum/rich_editor.rs | 40% ⚠️ Low |
| SearchBar | src/components/forum/search_bar.rs | 40% ⚠️ Low |
| Sidebar | src/components/layout/sidebar.rs | 35% ⚠️ Low |
| SiteCustomization | src/components/forum/admin/site_customization.rs | 40% ⚠️ Low |
| SubscriptionButton | src/components/forum/topics/subscription_button.rs | 40% ⚠️ Low |
| SyncStatus | src/components/sync_status.rs | 25% ⚠️ Low |
| TagAnalytics | src/components/forum/tag_analytics.rs | 40% ⚠️ Low |
| TagBrowser | src/components/forum/tag_browser.rs | 40% ⚠️ Low |
| TagCloud | src/components/forum/tag_cloud.rs | 40% ⚠️ Low |
| TagDetail | src/components/forum/tag_detail.rs | 40% ⚠️ Low |
| TagFeed | src/components/forum/tag_feed.rs | 40% ⚠️ Low |
| TagFilter | src/components/forum/tag_filter.rs | 40% ⚠️ Low |
| TagFollowing | src/components/forum/tag_following.rs | 40% ⚠️ Low |
| TagManagement | src/components/forum/tag_management.rs | 40% ⚠️ Low |
| TagSelector | src/components/forum/tag_selector.rs | 40% ⚠️ Low |
| ThreadDetail | src/components/forum/thread_detail.rs | 40% ⚠️ Low |
| ThreadDetail | src/components/posts.rs | 40% ⚠️ Low |
| TopicForm | src/components/forum/topic_form.rs | 40% ⚠️ Low |
| TopicForm | src/components/topics.rs | 55%  |
| TopicsList | src/components/topics.rs | 35% ⚠️ Low |
| UserGroups | src/components/forum/admin/user_groups.rs | 40% ⚠️ Low |
| UserManagement | src/components/forum/admin/user_management.rs | 25% ⚠️ Low |
| UserProfile | src/components/auth.rs | 25% ⚠️ Low |
| UserProfile | src/components/forum/user_profile.rs | 25% ⚠️ Low |
| UserSubscriptions | src/components/forum/user/subscriptions.rs | 40% ⚠️ Low |

### Code Quality Metrics

| Metric              | Value |
|---------------------|-------|
| Avg Complexity      | 4.1 |
| High Complexity Files | 431 |
| Technical Debt Score| 0% |



## 📊 SOLID Principles Violations

### Single Responsibility Principle (0 violations)

No SRP violations detected.
## 📊 SOLID Principles Violations

| Principle | Violations | Most Affected Component |
|-----------|------------|------------------------|
| Single Responsibility | 0 | - |
| Open-Closed | 0 | - |
| Liskov Substitution | 0 | - |
| Interface Segregation | 0 | - |
| Dependency Inversion | 0 | - |

*For detailed analysis, see [SOLID Code Smells Report](docs/solid_code_smells.md)*



## 📈 Project Trajectories (Predictions)

```json
{
  "models": {
    "remaining": 4,
    "weeks": 2.6666666666666665,
    "date": "2025-04-23"
  },
  "apiEndpoints": {
    "remaining": 67,
    "weeks": 22.333333333333332,
    "date": "2025-09-08"
  },
  "uiComponents": {
    "remaining": 11,
    "weeks": 2.2,
    "date": "2025-04-20"
  },
  "project": {
    "weeks": 22.333333333333332,
    "date": "2025-09-08"
  }
}
```
