name: Auto Merge PR

on:
  pull_request_target:
    types: [opened, synchronize, reopened]

jobs:
  auto-merge:
    if: |
      github.event.pull_request.user.login == '49016' &&
      contains(github.event.pull_request.title, '[merge]')
    runs-on: ubuntu-latest
    permissions:
      contents: write
      pull-requests: write
    steps:
      - name: Checkout main branch
        uses: actions/checkout@v4
        with:
          ref: main
          token: ${{ secrets.GITHUB_TOKEN }}
          fetch-depth: 0

      - name: Configure Git
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
      - name: Get PR commits and create patch
        id: get-patch
        run: |
          # Get the PR head commit SHA
          PR_HEAD_SHA="${{ github.event.pull_request.head.sha }}"
          PR_BASE_SHA="${{ github.event.pull_request.base.sha }}"
          
          echo "PR Head SHA: $PR_HEAD_SHA"
          echo "PR Base SHA: $PR_BASE_SHA"
          
          # Fetch the PR branch
          git fetch origin pull/${{ github.event.number }}/head:pr-branch
          
          # Create patch from the PR commits
          git format-patch $PR_BASE_SHA..pr-branch --stdout > pr-changes.patch
          
          echo "Patch created successfully"
          echo "patch-file=pr-changes.patch" >> $GITHUB_OUTPUT
      - name: Apply patch to main branch
        run: |
          # Apply the patch
          if git apply --check pr-changes.patch; then
            echo "Patch can be applied cleanly"
            git apply pr-changes.patch
          else
            echo "Patch conflicts detected, attempting 3-way merge"
            git apply --3way pr-changes.patch || {
              echo "Failed to apply patch even with 3-way merge"
              exit 1
            }
          fi
          
          # Stage all changes
          git add .
      - name: Commit and force merge
        run: |
          # Check if there are any changes to commit
          if git diff --staged --quiet; then
            echo "No changes to commit"
          else
            # Commit the changes
            git commit -m "Auto-merge: ${{ github.event.pull_request.title }}" \
                      -m "Merged from PR #${{ github.event.number }}" \
                      -m "Original author: ${{ github.event.pull_request.user.login }}"
            
            # Force push to main (force merge)
            git push origin main --force-with-lease
            
            echo "Successfully merged and pushed to main branch"
          fi
      - name: Close PR
        uses: actions/github-script@v7
        with:
          script: |
            await github.rest.pulls.update({
              owner: context.repo.owner,
              repo: context.repo.repo,
              pull_number: ${{ github.event.number }},
              state: 'closed'
            });
            
            await github.rest.issues.createComment({
              owner: context.repo.owner,
              repo: context.repo.repo,
              issue_number: ${{ github.event.number }},
              body: '✅ Auto-merged successfully! Changes have been applied to the main branch with force merge.'
            });
