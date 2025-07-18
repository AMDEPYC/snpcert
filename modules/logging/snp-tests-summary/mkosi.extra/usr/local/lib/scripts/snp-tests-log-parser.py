import sys
import json
import subprocess
import importlib
from collections import defaultdict
import emoji as em
import datetime
import re

# Constants
test_status_emojis = {
    'done': em.emojize(':check_mark_button:'),
    'failed': em.emojize(':cross_mark:'),
    'skipped': em.emojize(':fast_forward:', language='alias')
}

guest_journal_location="/var/log/journal/guest-logs/"

# Utility Functions
def fix_message_format(message):
    """Decode if the journal message is a list of character codes."""

    if isinstance(message, list):
        try:
            message = bytes(message).decode("utf-8", "replace")
        except Exception:
            message = ""
    return message

def human_readable_timestamp(microseconds):
    """Convert microseconds to a human-readable timestamp."""

    timestamp = int(microseconds) / 1_000_000
    dt = datetime.datetime.fromtimestamp(timestamp, datetime.UTC)
    return dt.strftime('%Y-%m-%d %H:%M:%S.%f UTC')

def run_journalctl_command(command_args):
    """Runs the given journalctl command and returns the output line iterator."""

    proc = subprocess.Popen(
        command_args,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        bufsize=1
    )
    return proc

def parse_journal_entries(proc):
    """Reads lines from the given subprocess, parses them as JSON, and returns a list of entries."""

    entries = []
    for line in proc.stdout:
        line = line.strip()
        if not line:
            continue
        try:
            entry = json.loads(line)
            entries.append(entry)
        except Exception:
            continue

    proc.stdout.close()
    proc.wait()
    return entries

def get_service_description(snp_test_service,platform):
    """Get the SNP host/guest test service description."""

    match platform:
        case "host" :
            host_serv_desc_cmd = f"journalctl  -u {snp_test_service} -o json | jq -r '.MESSAGE' | grep -i start | head -1"
            service_command = host_serv_desc_cmd

        case "guest" :
            guest_serv_desc_cmd = f"journalctl -D {guest_journal_location} -u {snp_test_service} -o json | jq -r '.MESSAGE' | grep -i start | head -1"
            service_command = guest_serv_desc_cmd

    command = subprocess.run(service_command, shell=True, check=True, text=True, capture_output=True)
    status_description = command.stdout

    match = re.split(r'(?i)-\s+', status_description, maxsplit=1)
    service_description=match[1].strip()

    return service_description

# SNP Test Journal Functions
def get_snp_test_journal_entries():
    """Collect and return all the SNP test journal entries with subprocess."""

    command_args = ["journalctl", "TEST=TRUE", "-o", "json"]
    proc = run_journalctl_command(command_args)
    return parse_journal_entries(proc)

def gather_snp_test_statuses(entries):
    """Extract all the SNP test service statuses."""

    snp_test_result_entries = [obj for obj in entries if "JOB_RESULT" in obj]
    return {entry["UNIT"]: entry["JOB_RESULT"] for entry in snp_test_result_entries}

def get_snp_host_test_summary():
    """Collect and return all the SNP host test journal entries."""

    proc = run_journalctl_command(["journalctl", "SNP_HOST_TEST_LOG=TRUE", "-o", "json"])
    test_entries = parse_journal_entries(proc)

    test_statuses = gather_snp_test_statuses(test_entries)
    overall_test_emoji = 'done'
    test_content = ''
    for test_service, test_status in test_statuses.items():
        test_status_emoji = test_status_emojis.get(test_status.lower(), '?')
        test_content += "\t" + f"{test_status_emoji} {test_service} :"
        test_description = get_service_description(test_service,"host")
        test_content += "  " + test_description + "\n"

        # Set overall SNP host test status emoji based on the single failed/skipped SNP test
        if test_status.lower() == 'failed':
            overall_test_emoji = 'failed'
        elif test_status.lower() == 'skipped':
            overall_test_emoji = 'skipped'
        else:
            overall_test_emoji = 'done'

    overall_test_status = "[ " + test_status_emojis.get(overall_test_emoji, '?') + " ] SNP HOST TESTS \n"

    snp_host_test_summary = overall_test_status + test_content
    return snp_host_test_summary

def get_snp_guest_attestation_summary():
    """Execute SNP Guest journalctl command and return the processed message content."""

    # Get journal entries with SNP Guest Attestation test results
    guest_attestation_service="fetch-snpguest-attestation-status.service"
    snpguest_attestation_cmd = f"journalctl -D {guest_journal_location} -u {guest_attestation_service} -o json | jq -r '.MESSAGE'"
    snpguest_attestation_result = subprocess.run(snpguest_attestation_cmd, shell=True, check=True, text=True, capture_output=True)

    # Extract and parse JSON objects from output
    json_objects = re.findall(r'\{[^}]+\}', snpguest_attestation_result.stdout)

    snpguest_attestation_data = {}
    for obj in json_objects:
        snpguest_attestation_data.update(json.loads(obj))

    # Convert status codes to human-readable form
    for snpguest_step, status_code in snpguest_attestation_data.items():
        snpguest_attestation_data[snpguest_step] = "done" if int(status_code) == 0 else "failed"

    # Format output with status emojis
    snpguest_attestation_summary = ''
    for step, step_status in snpguest_attestation_data.items():
        emoji = test_status_emojis.get(step_status.lower(), '?')
        snpguest_attestation_summary += "\t\t\t " + f"{emoji} {step}" + "\n"

    return snpguest_attestation_summary

def get_snp_guest_test_summary():
    """Collect and return all the SNP host test journal entries."""

    proc = run_journalctl_command(["journalctl","-D" , guest_journal_location ,  "SNP_GUEST_TEST_LOG=TRUE" , "-o", "json"])
    test_entries = parse_journal_entries(proc)
    test_statuses = gather_snp_test_statuses(test_entries)
    overall_test_emoji = 'done'
    attestation_summary= get_snp_guest_attestation_summary() + "\n"

    test_content = ''
    for test_service, test_status in test_statuses.items():
        test_status_emoji = test_status_emojis.get(test_status.lower(), '?')
        test_content += "\t" + f"{test_status_emoji} {test_service} :"
        test_description = get_service_description(test_service,"guest")
        test_content += "  " + test_description + "\n"

        # Add step-by-step summary status of the attestation workflow
        if "snpguest-attestation.service" in test_service.lower() :
            test_content += attestation_summary

        # Determine the emoji for the overall SNP guest test status based on single failed/skipped SNP test
        if test_status.lower() == 'failed':
            overall_test_emoji = 'failed'
        elif test_status.lower() == 'skipped':
            overall_test_emoji = 'skipped'
        else:
            overall_test_emoji = 'done'

    overall_test_status = "[ " + test_status_emojis.get(overall_test_emoji, '?') + " ] SNP GUEST TESTS \n"
    snpguest_test_summary = overall_test_status + test_content

    return snpguest_test_summary

# SNP Test Summary and Log Generation
def generate_snp_test_summary():
    """Display SNP test status summary with emojis."""

    test_summary_title = "\n=== SNP Certification Test Results === \n"

    test_summary_content = ''
    test_summary_content += get_snp_host_test_summary() + "\n"
    test_summary_content += get_snp_guest_test_summary()

    snp_test_summary = test_summary_title + "\n" + test_summary_content

    return snp_test_summary

def generate_snp_log_message(snp_test_journal_entries):
    """Generate a complete log message from SNP test journal entries."""

    snp_log_message_title = "\n=== View the complete SNP test log === \n"

    snp_log_lines = []
    for entry in snp_test_journal_entries:
        raw_timestamp = entry.get('__REALTIME_TIMESTAMP', 0)
        readable_timestamp = human_readable_timestamp(raw_timestamp)

        # Get and fix the log message
        message = fix_message_format(entry.get('MESSAGE', ''))

        # Show the log line
        log_line = f"[{readable_timestamp}] {message}"
        snp_log_lines.append(log_line)

    snp_log_message_content = "\n".join(snp_log_lines)

    snp_log_message = snp_log_message_title + "\n" + snp_log_message_content
    return snp_log_message

# Main Function
def main():
    """Main function to run the SNP test log parser."""

    snp_test_summary = generate_snp_test_summary()
    print(snp_test_summary)

    snp_test_journal_entries = get_snp_test_journal_entries()
    snp_log_message = generate_snp_log_message(snp_test_journal_entries)
    print(snp_log_message)

    snp_test = snp_test_summary + "\n" + snp_log_message

    pastebin_service_message = f"echo '{snp_test}' | aha | html2text | fpaste"
    pastebin_service_message = pastebin_service_message.expandtabs(tabsize=2)

    print("\nSNP Certification results are posted at: \n")
    subprocess.run(pastebin_service_message, shell=True, check=True)

if __name__ == "__main__":
    main()
