# ELM - OpenBadges v3 testing

## Overview
To do a quick test of the translation run from the credential-converter directory

cargo run -- -i ./json/ebsi-elm/vcdm2.0-europass-edc-schema/examples/DigiComp_Generic.json -o ./json/output/example_ELM_to_OBv3.json -m ./json/mapping/custom_mapping_ELM_OBv3_latest.json -c ELMtoOBv3

cargo run -- -i ./json/ebsi-elm/vcdm2.0-europass-edc-schema/examples/Bengales_highSchoolDiploma.json -o ./json/output/example_ELM_to_OBv3.json -m ./json/mapping/custom_mapping_ELM_OBv3_latest.json -c ELMtoOBv3


cargo run -- -i ./json/output/example_ELM_to_OBv3.json -o ./json/output/example_OBv3_to_ELM.json -m ./json/mapping/custom_mapping_OBv3_ELM_latest.json -c OBv3toELM