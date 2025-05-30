#!/bin/bash

# Environment Variables
ATTESTATION_DIR="/usr/attestation_service"

snpguest_regular_attestation_workflow() {

  # Run SNP tests
  snpguest ok

  # Cleanup and create a fresh attestation directory
  [  -d "${ATTESTATION_DIR}" ] &&  rm -rf "${ATTESTATION_DIR}"
  mkdir -p "${ATTESTATION_DIR}"

  # Request SNP Attestation Report(Version: 3) with random data
  snpguest report ${ATTESTATION_DIR}/attestation-report.bin ${ATTESTATION_DIR}/random-request-data.txt --random

  # Fetch ARK, ASK, VCEK certificates (saved in ./certificates)
  snpguest fetch ca pem -r ${ATTESTATION_DIR}/attestation-report.bin ${ATTESTATION_DIR}/certificates
  snpguest fetch vcek pem ${ATTESTATION_DIR}/certificates/ ${ATTESTATION_DIR}/attestation-report.bin

  # Verify if ARK, ASK and VCEK are all signed properly
  snpguest verify certs ${ATTESTATION_DIR}/certificates/
  snpguest verify attestation ${ATTESTATION_DIR}/certificates/ ${ATTESTATION_DIR}/attestation-report.bin

  # Display the SNP Attestation Report(Version: 3) with random data
  snpguest display report ${ATTESTATION_DIR}/attestation-report.bin
}

validate_request_data(){
  local random_request_data="$(tr -d '\n'< "${ATTESTATION_DIR}/random-request-data.txt")"

  echo -e "\n"
  echo -e "Random Request Data:"
  echo -e "${random_request_data}"

  # Get the measurement attribute from the SNP Attestation Report
  local snpguest_report_request_data=$(snpguest display report ${ATTESTATION_DIR}/attestation-report.bin \
											| tr '\n' ' ' | sed "s|.*Report Data:\(.*\)Measurement.*|\1\n|g" \
											| sed "s| ||g" \
											| tr '[:upper:]' '[:lower:]'
										)
  snpguest_report_request_data=$(echo ${snpguest_report_request_data} | sed $'s/[^[:print:]\t]//g')

  echo -e "\n"
  echo -e "Request Data from SNP Attestation Report:"
  echo -e "${snpguest_report_request_data} \n"

  # Compare the expected request data to the guest report request data
  [[ "${random_request_data}" == "${snpguest_report_request_data}" ]] \
    && echo -e "The random request data generated matches the snp guest report request data!"    \
    || { >&2 echo -e "FAIL: Request data do not match"   ; return 1; }
}

validate_snp_measurement(){
  # Get the expected measurement from Host Data Attribute in SNP Attestation Report
  local expected_measurement=$(snpguest display report ${ATTESTATION_DIR}/attestation-report.bin \
											| tr '\n' ' ' | sed "s|.*Host Data:\(.*\)ID Key Digest:.*|\1\n|g" \
											| sed "s| ||g" \
											| tr '[:upper:]' '[:lower:]'
										)
  expected_measurement=$(echo ${expected_measurement} | sed $'s/[^[:print:]\t]//g')

  echo -e "\n"
  echo -e "Expected Measurement:"
  echo -e "${expected_measurement}"

  # Get the SHA-256 sum of the measurement attribute from the SNP Attestation Report
  local snpguest_report_measurement=$(snpguest display report ${ATTESTATION_DIR}/attestation-report.bin \
											| tr '\n' ' ' | sed "s|.*Measurement:\(.*\)Host Data.*|\1\n|g" \
											| sed "s| ||g" \
											| tr '[:upper:]' '[:lower:]'
										)
  #snpguest_report_measurement=$(echo ${snpguest_report_measurement} | sed $'s/[^[:print:]\t]//g')
  snpguest_report_measurement=$(echo ${snpguest_report_measurement} | sha256sum | cut -d ' ' -f 1 )

  echo -e "\n"
  echo -e "Measurement from SNP Attestation Report:"
  echo -e "${snpguest_report_measurement} \n"

  # Compare the expected measurement with the guest report measurement
  [[ "${expected_measurement}" == "${snpguest_report_measurement}" ]] \
	&& echo -e "The expected measurement matches the snp guest report measurement!" \
	|| { >&2 echo -e "FAIL: measurements do not match"; return 1; }
}

main() {
  echo -e "Perform Regular Attestation workflow using snpguest tool ..."
  snpguest_regular_attestation_workflow

  echo -e "\n"
  echo -e "Validate Request Data Attribute ..."
  validate_request_data

  echo -e "\n"
  echo -e "Validate Measurement Attribute ..."
  validate_snp_measurement
}

main
