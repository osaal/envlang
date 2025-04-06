**Welcome to the `Envlang` wiki!**

The wiki contains information for both users and developers of Envlang.

# Editing the Wiki

The Wiki is built from Markdown documents residing in the `/wiki` folder of the source repository using a GitHub Action. Writing to the Wiki is done by editing `.md` files inside the source repository and pull-requesting them into `main`, which triggers the Action to update the Wiki.

**No updates to the Wiki are done through the public GitHub interface or by directly editing the Wiki files outside of the source repository.** Such updates will be blocked or reverted.

If you want to contribute to the Wiki, follow these steps:

1. Clone the source repository and make your preferred changes to the `/wiki` files.
2. Commit and open a pull request using the "Wiki" pull request template. Wiki edits do not need to be documented in the `CHANGELOG`, and are not considered when numbering Envlang versions. You also do not need to add any Labels or Milestones, as the PR template automatically labels Wiki PRs appropriately.
3. The PR is reviewed by the maintainers, approved, and pulled into `main` (unless egregiously false, unreadable, irrelevant, etc.).
4. The Action is triggered and the Wiki is automatically updated.