# Contributing Guide
We welcome contributions from the community! If you would like to participate in the project, here are some things to consider.

## Code of Conduct
Please read and follow the [Code of Conduct](./CODE_OF_CONDUCT.md).

## Topics

* [Reporting Issues](#reporting-issues)
* [Submitting Pull Requests](#submitting-pull-requests)
* [Submission Guidelines](#submission-guidelines)
* [Workflow](#workflow)

## Reporting Issues

Before reporting an issue, check our backlog of Open Issues to see if someone else has already reported it.
If so, feel free to add your scenario, or additional information, to the discussion.
Or simply "subscribe" to it to be notified when it is updated.
Please do not add comments like "+1" or "I have this issue as well" without adding any new information.
Instead, please add a thumbs-up emoji to the original report.

Note: Older closed issues/PRs are automatically locked.
If you have a similar problem please open a new issue instead of commenting.

If you find a new issue with the project we'd love to hear about it!

The most important aspect of a bug report is that it includes enough information for us to reproduce it.
Please include as much detail as possible, including all requested fields in the template.
Not having all requested information makes it much harder to find and fix issues.
A reproducer is the best thing you can include.
Reproducers make finding and fixing issues much easier for maintainers.
The easier it is for us to reproduce a bug, the faster it'll be fixed!

Please don't include any private/sensitive information in your issue!
Security bugs should NOT be reported via Github and should instead be reported via the process described [here](SECURITY.md).

## Submitting Pull Requests

No Pull Request (PR) is too small!
Typos, additional comments in the code, new test cases, bug fixes, new features, more documentation, ... it's all welcome!

Our projects follow the normal GitHub PR workflow for contributions.
If you never worked with GitHub and git before you likely first need to understand some basic about them.
The general work you have to do when you contribute the first time is something like this:
 - Fork the project on GitHub.
 - Clone that fork locally.
 - Create a new branch.
 - Make your change and commit it.
 - Build your host/guest image with mkosi tool, and test the host/guest image boot on your system,
 - Push the branch to your fork.
 - Open a PR against the upstream repo with your host/guest image boot test logs, screenshots/test results in the PR comments.

You can find some easy tutorial online such as [this one](https://opensource.com/article/19/7/create-pull-request-github)
and check out the official [GitHub docs](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests)
that contain much more detail.

All development happens on the `main` branch so all PRs should be submitted against that branch.
Maintainers will take care of backporting if needed.

While bug fixes can first be identified via an "issue" in Github, that is not required.
It's ok to just open up a PR with the fix, but make sure you include the same information you would have included in an issue - like how to reproduce it.

PRs for new features should include some background on what use cases the new code is trying to address.
When possible and when it makes sense, try to break-up larger PRs into smaller ones - it's easier to review smaller code changes.
But only if those smaller ones make sense as stand-alone PRs.

Regardless of the type of PR, all PRs should include a well-documented code changes, both through comments in the code itself.

Cosider below points for the high-quality commit messages:
- A commit message should be drafted as per the [conventional commit style](https://www.conventionalcommits.org/en/v1.0.0/)
- A commit description should answer *why* a change was made.

Squash your commits into logical pieces of work that might want to be reviewed separate from the rest of the PRs.
Code changes, test and documentation updates should be part of the same commit as long as they are for the same
feature/bug fix. Dependency updates are best kept in an individual commit. Totally unrelated changes, i.e.
fixing typos in a different code part or adding a completely different feature should go into their own PR.
Often squashing down to just one commit is acceptable since in the end the entire PR will be reviewed anyway.
When in doubt, ask a maintainer how they prefer it.

This repository follows a main branch protection rule for merges.
PRs will be approved by the maintainers of this repository.
They will then be merged by a repo owner. A review is required for a pull request to merge.

### Sign your PRs

The sign-off is a line at the end of the commit message.
Your signature certifies that you wrote the commit.

Use a real name (sorry, no anonymous contributions).
If you set your `user.name` and `user.email` git configs, you can sign your commit automatically with `git commit -s`.

### Code review

Once the PR is submitted a reviewer will take a look at.
Should nobody respond to it within 2 weeks please ping a maintainer.
Sometimes PRs are overlooked or forgotten.

Keep an eye out for the CI results on the PR.
If all is well then all tasks should succeed.
On some repos the CI tests can take several minutes to finish.
If something failed, try to take a look at the logs to see if that seems related to your change or not.
Then try to fix your code or the test depending on what you think is right.
If you are unsure or think it is unrelated, ask a maintainer.
Some tests are flaky and will pass on a re-run.

After the reviewers and maintainers take a look, they will either write a comment stating `LGTM` (looks good to me) and approve the PR, in which case you do not need to do any further changes, or they write a comment with review feedback that you should address.

If changes were requested, make them locally in your branch and the amend them into the commit from the PR.
Please do not push extra commits that say things like "apply code review" or "fix x" where x is a bug introduced in a commit from your PR.

Squash the change into the right commit to keep the git history clean.
Our projects merge the commits as is and will will not squash them on merge to preserve the full original context.

## Submission Guidelines

###  Submitting an Issue
Before starting work on a pull request, please check if there is an open issue that your contribution relates to. 
If there is no such issue, please create a new issue to describe your contribution and start the discussion.


### Submitting a Pull Request (PR)
When you are ready to submit your contribution, please create a pull request.
Here are some things to consider:

- Add a descriptive title to the pull request.
- Link the related issue in the pull request.
- Ensure that all tests are successful and no warnings occur.
- Use a descriptive commit message.

## Workflow

- Find an issue that you would like to work on or create a new issue to propose a new feature or improvement.
- Fork the repository on GitHub.
- Create a new branch for your changes.
- Make your changes and commit them to your branch.
- Build your host/guest image with mkosi tool, and test the host/guest image boot on your system,
- Push your branch to your fork on GitHub.
- Create a new pull request from your branch with your host/guest image boot test logs, screenshots/test results in the PR  comments.

A maintainer will review your pull request and may ask you to make additional changes
or provide more information before it is merged.

## License
By submitting a contribution, you agree to have your contribution published under the project's license. 
Please make sure you have the right to submit your contribution under this license.

## Acknowledgements
Thank you for wanting to contribute to the project! We appreciate the effort and time you are putting into your contribution.
