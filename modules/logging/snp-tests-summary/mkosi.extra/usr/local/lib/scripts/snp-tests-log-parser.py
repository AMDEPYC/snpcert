import sys
import json
import subprocess
import importlib
from collections import defaultdict
import emoji as em
import datetime

# Constants
test_status_emojis = {
    'done': em.emojize(':check_mark_button:'),
    'failed': em.emojize(':cross_mark:'),
    'skipped': em.emojize(':fast_forward:', language='alias')
}

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
    dt = datetime.datetime.utcfromtimestamp(timestamp)
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

def get_service_description(snp_test_service):
    """Get the SNP test service description."""
    try:
        snp_test_description = subprocess.check_output(['systemctl', 'cat', snp_test_service], text=True)
        for line in snp_test_description.splitlines():
            if line.startswith('Description='):
                return line.split('=', 1)[1].strip()
        return "No description available"
    except subprocess.CalledProcessError:
        return "Failed to retrieve description"

# SNP Test Journal Functions
def get_snp_test_journal_entries():
    """Collect and return all the SNP test journal entries with subprocess."""
    command_args = ["journalctl", "TEST=TRUE", "-o", "json"]
    proc = run_journalctl_command(command_args)
    return parse_journal_entries(proc)

def gather_snp_test_statuses(snp_test_journal_entries):
    """Extract all the SNP test service statuses."""
    snp_test_result_entries = [obj for obj in snp_test_journal_entries if "JOB_RESULT" in obj]
    return {entry["UNIT"]: entry["JOB_RESULT"] for entry in snp_test_result_entries}

def get_snp_host_test_journal_summary():
    """Collect and return all the SNP host test journal entries."""

    proc = run_journalctl_command(["journalctl", "SNP_HOST_TEST_LOG=TRUE", "-o", "json"])
    snp_host_test_entries = parse_journal_entries(proc)

    snp_host_test_statuses = gather_snp_test_statuses(snp_host_test_entries)
    overall_snp_host_test_emoji = 'done'
    snp_host_test_content = ''
    for snp_test_service, snp_test_status in snp_host_test_statuses.items():
        snp_test_status_emoji = test_status_emojis.get(snp_test_status.lower(), '?')
        snp_host_test_content += f"    {snp_test_status_emoji} {snp_test_service}: "
        snp_host_test_description = get_service_description(snp_test_service)
        snp_host_test_content += " " + snp_host_test_description + "\n"

        # Determine the emoji for the overall SNP host test status
        if snp_test_status.lower() == 'failed':
            overall_snp_host_test_emoji = 'failed'
        elif snp_test_status.lower() == 'skipped':
            overall_snp_host_test_emoji = 'skipped'
        else:
            overall_snp_host_test_emoji = 'done'

    test_title = "=== SNP Host Test  === \n"
    overall_test_status = "[ " + test_status_emojis.get(overall_snp_host_test_emoji, '?') + " ] SNP HOST TESTS \n"

    snp_host_test = test_title + overall_test_status + snp_host_test_content
    return snp_host_test

# SNP Test Summary and Log Generation
def generate_snp_test_summary_content():
    """Display SNP test status summary with emojis."""

    snp_test_summary_content = ''
    snp_test_summary_content += get_snp_host_test_journal_summary()

    return snp_test_summary_content

def generate_snp_complete_log_message(snp_test_journal_entries):
    """Generate a complete log message from SNP test journal entries."""

    snp_log_lines = []
    for entry in snp_test_journal_entries:
        raw_timestamp = entry.get('__REALTIME_TIMESTAMP', 0)
        readable_timestamp = human_readable_timestamp(raw_timestamp)

        # Get and fix the log message
        message = fix_message_format(entry.get('MESSAGE', ''))

        # Show the log line
        log_line = f"[{readable_timestamp}] {message}"
        snp_log_lines.append(log_line)

    return "\n".join(snp_log_lines)

# Main Function
def main():
    """Main function to run the SNP test log parser."""

    snp_test_summary_title = "\n=== SNP Certification Test Results === \n"
    snp_test_summary_content = generate_snp_test_summary_content()

    snp_test_summary = snp_test_summary_title + "\n" + snp_test_summary_content
    print(snp_test_summary)

    snp_log_message_title = "\n=== View the complete SNP test log === \n"
    snp_test_journal_entries = get_snp_test_journal_entries()
    snp_log_message_content = generate_snp_complete_log_message(snp_test_journal_entries)

    snp_log_message = snp_log_message_title + "\n" + snp_log_message_content
    print(snp_log_message)

    snp_test_result = snp_test_summary + "\n" + snp_log_message
    pastebin_service_message = f"echo '{snp_test_result}' | aha | html2text | fpaste"

    print("\nSNP Certification results are posted at: \n")
    subprocess.run(pastebin_service_message, shell=True, check=True)

if __name__ == "__main__":
    main()
