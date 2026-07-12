# herdr Workspace Policy Implementation Plan

> **Execution:** Use `/execute-plan` to dispatch this plan to agent-teams. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** AE ワークフローの worktree 運用を herdr workspace 前提に移行する — `using-git-worktrees` スキルを `create-workspace` にリネーム・再設計し、全参照を更新し、方針を CLAUDE.global.md に明文化する。

**Architecture:** feature 作業は「1 feature = 1 git worktree(`~/.herdr/worktrees/<repo>/<branch>`)= 1 herdr workspace = 1 セッション」で design-discussion から finish-branch まで貫通する。`create-workspace` は (a) worktree 内なら検証+セットアップ、(b) main checkout なら質問の上 `herdr worktree create` を実行してエンジニアに新セッション開始を案内する。Claude 自発の worktree(subagent isolation / EnterWorktree)は `.claude/worktrees` のままスコープ外。workspace の削除はエンジニアの手動操作。

**Tech Stack:** Markdown(スキル定義・グローバル指示)のみ。コード変更なし。herdr CLI 0.7.1(socket API)を前提としたコマンド記述。

**Working directory:** `/Users/sakumatomoya/workspace/dotfiles`(全コマンドはリポジトリルートから実行)。
**Branch:** `feat/herdr-workspace-policy`(Task 1 の前に `git switch -c feat/herdr-workspace-policy` で作成。本方針はこのプランのマージ後に有効化されるため、今回の作業自体は従来どおり main checkout 上のブランチで行う)。
**Baseline before Task 1:** `git status` clean、`grep -rln "using-git-worktrees" claude/skills claude/CLAUDE.global.md` が `claude/skills/using-git-worktrees/SKILL.md`・`claude/skills/execute-plan/SKILL.md`・`claude/skills/agent-teams-driven-development/SKILL.md`・`claude/CLAUDE.global.md` の 4 ファイルを返すこと。

**Per-task verification command**(各コミット前に必須):
```sh
cd /Users/sakumatomoya/workspace/dotfiles && git diff --check && git status --short
```
Expected: 空白エラーなし、変更ファイルがそのタスクの Files 欄と一致。加えて各タスク固有の grep 検証(各タスクの Verify ステップ参照)。

**実測済みの事実**(プラン内コマンドの根拠、2026-07-13 検証):
- `herdr worktree create --cwd <repo-root> --branch <name> --no-focus --json` は worktree を `~/.herdr/worktrees/<repo名>/<ブランチ名>` に作成し、ブランチ名をラベルとする herdr workspace を開く。JSON の `.result.workspace.workspace_id` と `.result.worktree.path` が案内文に使える。
- worktree 内では `git rev-parse --path-format=absolute --git-common-dir` が本体の `.git` を返す(main checkout では `<toplevel>/.git` と一致)。
- セッション自身の workspace ID は環境変数 `HERDR_WORKSPACE_ID` で得られる。
- `claude/install.sh` は `rsync -a --delete` で skills/ を同期するため、スキルディレクトリのリネームは配布時に旧ディレクトリを自動削除する。install.sh の変更は不要。

---

### Task 1: `using-git-worktrees` → `create-workspace` リネームと全面書き換え

**Why:** スキルの実態が「git worktree の作成手順」から「feature 単位の herdr workspace を保証する」に変わる。worktree という実装詳細ではなく workspace という概念で命名し直し、本文を新方針(検証モード/作成モード/フォールバック)に置き換える。

**Behavior change:** yes(スキルの挙動定義の変更。実行可能テストはなく、検証は grep とファイル存在確認)
**Discipline:** doc edit — ステップごとの grep 検証

**Files:**
- Rename: `claude/skills/using-git-worktrees/` → `claude/skills/create-workspace/`
- Rewrite: `claude/skills/create-workspace/SKILL.md`(全文置き換え)

### Steps

- [ ] **Step 1: ディレクトリを git mv でリネーム**

```sh
cd /Users/sakumatomoya/workspace/dotfiles
git mv claude/skills/using-git-worktrees claude/skills/create-workspace
```

- [ ] **Step 2: SKILL.md を以下の内容で全文置き換え**

`claude/skills/create-workspace/SKILL.md` を次の内容で上書きする(Write で全置換):

`````markdown
---
name: create-workspace
description: |
  Ensure feature work runs in its own herdr workspace (a git worktree under
  ~/.herdr/worktrees). Verifies and sets up the workspace when the session is
  already inside one; when the session started in the main checkout, asks the
  engineer, creates the workspace via `herdr worktree create`, and guides the
  engineer to reopen there. Invoke with `/create-workspace`
  (called from /design-discussion and /execute-plan).
---

# Create Workspace

AE feature work runs in per-feature workspaces: one feature = one git worktree
under `~/.herdr/worktrees/<repo>/<branch>` = one herdr workspace = one Claude
session, from `/design-discussion` through `/finish-branch`. This skill ensures
that state.

**Core principle:** Claude runs the herdr commands; the engineer opens and
closes sessions.

**Announce at start:** "I'm using the create-workspace skill to set up the feature workspace."

## Scope

- Covers AE feature-work workspaces only. Worktrees Claude spawns autonomously
  (subagent `isolation: "worktree"`, EnterWorktree's managed location) stay
  harness-managed under `.claude/worktrees` and are not affected by this skill.
- Workspace removal is the engineer's manual operation
  (`herdr worktree remove`). No skill removes workspaces or deletes branches.

## Step 1: Detect Session Location

```bash
common=$(git rev-parse --path-format=absolute --git-common-dir)
toplevel=$(git rev-parse --show-toplevel)
```

| Result | Meaning | Next |
|---|---|---|
| `$common` ≠ `$toplevel/.git` | Linked worktree — feature workspace | Step 2 (verify & set up) |
| `$common` = `$toplevel/.git` | Main checkout — launchpad session | Step 3 (ask, then create) |

## Step 2: In a Worktree — Verify and Set Up

1. Confirm the branch: `git branch --show-current` must not be main/master.
2. Run project setup (auto-detect):

```bash
if [ -f package.json ]; then npm install; fi
if [ -f Cargo.toml ]; then cargo build; fi
if [ -f requirements.txt ]; then pip install -r requirements.txt; fi
if [ -f pyproject.toml ]; then poetry install; fi
if [ -f go.mod ]; then go mod download; fi
```

3. Verify clean baseline: run the project's test suite. If tests fail, report
   the failures and ask whether to proceed or investigate first.
4. Report:

```
Workspace ready at <path> (branch <branch>)
Tests passing (<N> tests, 0 failures)
```

## Step 3: In the Main Checkout — Ask, Then Create

Ask the engineer one question, with a proposed branch name derived from the
feature under discussion:

```
This session is in the main checkout. Feature work belongs in its own herdr
workspace.

1. Create workspace `<branch-name>` — I run `herdr worktree create`; you open
   a new session there and restart from /design-discussion. (recommended)
2. Continue here via EnterWorktree — keeps this conversation, but the feature
   does not get its own herdr workspace.

Which?
```

**Option 1 — create the workspace:**

```bash
herdr worktree create --cwd "$(git rev-parse --show-toplevel)" --branch <branch-name> --no-focus --json
```

- The JSON result carries `.result.workspace.workspace_id` and
  `.result.worktree.path` (`~/.herdr/worktrees/<repo>/<branch>`).
- Always `--no-focus`: never yank the engineer out of the current session.
- Do NOT run `herdr agent start` — starting the session is the engineer's act.

Then report and stop:

```
Workspace '<branch>' created at <path> (herdr workspace <id>).
Switch to it in herdr, run `claude`, and start with /design-discussion <topic>.
```

**Option 2 — EnterWorktree:** use the EnterWorktree tool and continue the flow
in this session.

## Fallback: herdr Unreachable

If the `herdr` CLI or its socket is unavailable, report that and ask the
engineer how to proceed (work on a feature branch in place, or the engineer
prepares a worktree manually). Do not reimplement worktree management with raw
git commands.

## Red Flags

| Violation | Correct Behavior |
|---|---|
| Creating AE worktrees with raw `git worktree add` | herdr owns AE workspaces. Use `herdr worktree create` (or the fallback question). |
| Running `herdr agent start` to launch the new session | The engineer opens sessions. Report and stop. |
| Creating the workspace with `--focus` | Never steal focus from the running session. |
| Removing workspaces or deleting worktree-checked-out branches | Removal is the engineer's manual operation. |
| Proceeding on main/master because "it's a small change" | Feature work gets a workspace. The engineer decides exceptions. |
| Skipping setup/baseline verification in a fresh worktree | Always verify before implementation starts. |

## Integration

**Called by:**
- `/design-discussion` — workspace check when the discussion reveals feature work (launchpad detection)
- `/execute-plan` — workspace precondition before dispatching to agent-teams
- `/agent-teams-driven-development` — workspace prerequisite

**Pairs with:**
- `/finish-branch` — completion; merge/discard are worktree-aware
- `/session-teardown` — session end; workspace removal stays with the engineer
`````

- [ ] **Step 3: Verify**

```sh
cd /Users/sakumatomoya/workspace/dotfiles
test -f claude/skills/create-workspace/SKILL.md && echo "DIR OK"
grep -q "name: create-workspace" claude/skills/create-workspace/SKILL.md && echo "NAME OK"
! grep -rq "using-git-worktrees" claude/skills/create-workspace/ && echo "NO STALE REF"
git diff --check && git status --short
```

Expected: `DIR OK` / `NAME OK` / `NO STALE REF`、変更は rename+rewrite のみ。

- [ ] **Step 4: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
skills: using-git-worktrees を create-workspace に再設計

feature 作業を herdr workspace(~/.herdr/worktrees の worktree)で行う
方針に合わせ、スキルを「worktree 作成手順」から「workspace 状態の保証」
に転換。worktree 内なら検証+セットアップ、main checkout なら質問の上
herdr worktree create を実行しエンジニアに新セッション開始を案内する。
ディレクトリ選択ロジック(.worktrees 探索 → CLAUDE.md grep → 質問)は
配置が herdr 管理に固定されたため削除。

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
EOF
)"
```

---

### Task 2: execute-plan / agent-teams-driven-development の参照更新

**Why:** 旧スキル名への参照が残ると、スキル起動が失敗する(rsync --delete で旧ディレクトリは配布先から消える)。呼び出し側 2 スキルの参照を新名称に差し替える。

**Behavior change:** no(参照名の差し替えのみ)
**Discipline:** doc edit — grep 検証

**Files:**
- Modify: `claude/skills/execute-plan/SKILL.md`(L18, L24, L59, L84)
- Modify: `claude/skills/agent-teams-driven-development/SKILL.md`(L24, L196)

### Steps

- [ ] **Step 1: execute-plan/SKILL.md の 4 箇所を Edit で置換**

| 旧(old_string) | 新(new_string) |
|---|---|
| `- A feature branch or worktree is set up (NOT main/master). If not, invoke `/using-git-worktrees` first.` | `- A feature workspace is set up (NOT main/master). If not, invoke `/create-workspace` first.` |
| `Confirm an isolated workspace (worktree or feature branch) is set up. If not, invoke `/using-git-worktrees`.` | `Confirm an isolated workspace (herdr worktree or feature branch) is set up. If not, invoke `/create-workspace`.` |
| `\| Executing on main/master \| Stop. Set up worktree or feature branch via /using-git-worktrees. \|` | `\| Executing on main/master \| Stop. Set up the feature workspace via /create-workspace. \|` |
| `- `/using-git-worktrees` — workspace setup before execution` | `- `/create-workspace` — workspace verification/setup before execution` |

- [ ] **Step 2: agent-teams-driven-development/SKILL.md の 2 箇所を Edit で置換**

| 旧(old_string) | 新(new_string) |
|---|---|
| `- An isolated workspace is set up (via `/using-git-worktrees`)` | `- An isolated workspace is set up (via `/create-workspace`)` |
| `- `/using-git-worktrees` — isolated workspace before starting` | `- `/create-workspace` — isolated workspace before starting` |

- [ ] **Step 3: Verify**

```sh
cd /Users/sakumatomoya/workspace/dotfiles
! grep -rq "using-git-worktrees" claude/skills/execute-plan claude/skills/agent-teams-driven-development && echo "REFS CLEAN"
grep -c "create-workspace" claude/skills/execute-plan/SKILL.md   # Expected: 4
grep -c "create-workspace" claude/skills/agent-teams-driven-development/SKILL.md   # Expected: 2
git diff --check && git status --short
```

Expected: `REFS CLEAN`、カウント 4 / 2。

- [ ] **Step 4: Commit**

```sh
git add claude/skills/execute-plan/SKILL.md claude/skills/agent-teams-driven-development/SKILL.md
git commit -m "$(cat <<'EOF'
skills: execute-plan / agent-teams の worktree スキル参照を create-workspace に更新

参照名の差し替えのみ。挙動変更なし。

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
EOF
)"
```

---

### Task 3: design-discussion に Workspace check を追加

**Why:** 新方針では design-discussion から workspace 内で行う。main checkout(launchpad セッション)で feature 作業の議論が始まった場合に早期に `/create-workspace` へ誘導するチェックを Operating Procedure に置く。議論が深まる前に検知するほど、開き直しの損失が小さい。

**Behavior change:** yes(スキルに手順を追加)
**Discipline:** doc edit — grep 検証

**Files:**
- Modify: `claude/skills/design-discussion/SKILL.md`(Operating Procedure リスト先頭に項目追加)

### Steps

- [ ] **Step 1: Operating Procedure リストの先頭に項目 0 を挿入**

`claude/skills/design-discussion/SKILL.md` 内のアンカー:

```
1. **Investigate first.** Once the topic is understood, your next
```

この行の直前に以下を挿入する(空行 1 つを挟む):

```markdown
0. **Workspace check.** As soon as the discussion reveals the work will
   change code — and no later than routing — check where this session
   runs: if `git rev-parse --path-format=absolute --git-common-dir`
   equals `<toplevel>/.git`, this is the main checkout (a launchpad
   session), so invoke `/create-workspace` before going deeper. Feature
   work lives in its own herdr workspace from design-discussion onward.
   Pure consultation, investigation, or prototyping support may stay
   where it is.
```

- [ ] **Step 2: Verify**

```sh
cd /Users/sakumatomoya/workspace/dotfiles
grep -q "Workspace check" claude/skills/design-discussion/SKILL.md && echo "CHECK ADDED"
grep -q "create-workspace" claude/skills/design-discussion/SKILL.md && echo "REF OK"
git diff --check && git status --short
```

Expected: `CHECK ADDED` / `REF OK`。

- [ ] **Step 3: Commit**

```sh
git add claude/skills/design-discussion/SKILL.md
git commit -m "$(cat <<'EOF'
skills: design-discussion に workspace check を追加

feature 作業と判明した時点で main checkout(launchpad)なら
/create-workspace へ誘導する項目 0 を Operating Procedure に追加。
議論が深まる前の検知で開き直しの損失を最小化する。

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
EOF
)"
```

---

### Task 4: finish-branch の Option 2 / Option 4 を worktree 対応にする

**Why:** 【設計議論後にプラン作成中に発見した追加項目 — エンジニアレビューで要確認】新方針では finish-branch が worktree セッション内で走る。現行の Option 2(local merge)の `git checkout <base-branch>` は base が本体 checkout に存在するため失敗し、Option 4(discard)の `git branch -D` は worktree がそのブランチをチェックアウトしている限り失敗する。worktree 内から本体 checkout に `-C` で操作する形に書き換え、削除系はエンジニアの workspace 削除に委ねる(「削除はエンジニアが行う」の決定と整合)。

**Behavior change:** yes(手順の書き換え)
**Discipline:** doc edit — grep 検証

**Files:**
- Modify: `claude/skills/finish-branch/SKILL.md`(Option 2 / Option 4 のブロック)

### Steps

- [ ] **Step 1: Option 2 ブロックを置換**

旧(old_string):

`````markdown
#### Option 2: Merge Locally

```bash
git checkout <base-branch>
git pull
git merge <feature-branch>
# Verify tests on merged result
git branch -d <feature-branch>
```
`````

新(new_string):

`````markdown
#### Option 2: Merge Locally

In a worktree session, `git checkout <base-branch>` is impossible — the base
branch is checked out in the main checkout. Operate on the main checkout via
`-C` instead:

```bash
main_root=$(dirname "$(git rev-parse --path-format=absolute --git-common-dir)")
git -C "$main_root" pull
git -C "$main_root" merge <feature-branch>
# Verify tests on the merged result (run them in $main_root)
```

The feature branch cannot be deleted while this worktree has it checked out.
Report instead:

```
Merged into <base-branch>. After you remove this workspace
(herdr worktree remove), delete the branch with `git branch -d <feature-branch>`.
```

(On a plain feature branch — no worktree — the classic sequence applies:
`git checkout <base-branch> && git pull && git merge <feature-branch>`,
verify tests, then `git branch -d <feature-branch>`.)
`````

- [ ] **Step 2: Option 4 の確認後ブロックを置換**

旧(old_string):

`````markdown
Wait for exact confirmation. If confirmed:
```bash
git checkout <base-branch>
git branch -D <feature-branch>
```
`````

新(new_string):

`````markdown
Wait for exact confirmation. If confirmed, in a worktree session Claude
deletes nothing — the branch is checked out here. Report instead
(`$HERDR_WORKSPACE_ID` carries this session's workspace ID):

```
To discard: remove this workspace
  herdr worktree remove --workspace $HERDR_WORKSPACE_ID --force
then delete the branch from the main checkout:
  git branch -D <feature-branch>
```

(On a plain feature branch — no worktree:
`git checkout <base-branch> && git branch -D <feature-branch>` as before.)
`````

- [ ] **Step 3: Verify**

```sh
cd /Users/sakumatomoya/workspace/dotfiles
grep -q "git-common-dir" claude/skills/finish-branch/SKILL.md && echo "OPT2 OK"
grep -q "HERDR_WORKSPACE_ID" claude/skills/finish-branch/SKILL.md && echo "OPT4 OK"
! grep -q "^git checkout <base-branch>$" claude/skills/finish-branch/SKILL.md && echo "OLD GONE"
git diff --check && git status --short
```

Expected: `OPT2 OK` / `OPT4 OK` / `OLD GONE`。

- [ ] **Step 4: Commit**

```sh
git add claude/skills/finish-branch/SKILL.md
git commit -m "$(cat <<'EOF'
skills: finish-branch の local merge / discard を worktree セッション対応に

worktree 内では base への checkout とチェックアウト中ブランチの削除が
不可能。merge は本体 checkout への -C 操作に変更し、ブランチ/workspace
の削除はエンジニアの手動操作(herdr worktree remove)に委ねる。

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
EOF
)"
```

---

### Task 5: CLAUDE.global.md に Workspaces 方針を明文化

**Why:** 方針の SSOT。スキルは機構を持つが、契約(どこで作業し、誰が何を操作するか)は global に置く — 既存の「契約=global/機構=スキル」の設計に従う。

**Behavior change:** yes(グローバル契約の追加)
**Discipline:** doc edit — grep 検証

**Files:**
- Modify: `claude/CLAUDE.global.md`(`### Entry Point` の後に `### Workspaces` 節を追加、Cross-cutting Skills リストの 1 行差し替え)

### Steps

- [ ] **Step 1: `### Workspaces` 節を挿入**

アンカー(old_string):

```markdown
### Cross-cutting Skills

Invoked within other skills as needed, not as part of the core flow:
```

これを以下に置換(new_string — 節を前置):

```markdown
### Workspaces

Feature work runs in a per-feature herdr workspace: one feature = one git worktree under `~/.herdr/worktrees/<repo>/<branch>` = one session, from `design-discussion` through `finish-branch`. `create-workspace` verifies or establishes this state — Claude runs the herdr commands (`herdr worktree create`); opening the session in the new workspace and removing the workspace afterward (`herdr worktree remove`) are the engineer's actions. Worktrees Claude spawns autonomously (subagent `isolation: "worktree"`, EnterWorktree) stay harness-managed under `.claude/worktrees` and are outside this policy.

### Cross-cutting Skills

Invoked within other skills as needed, not as part of the core flow:
```

- [ ] **Step 2: Cross-cutting Skills リストの行を差し替え**

旧(old_string):

```markdown
- `using-git-worktrees` — invoked before `execute-plan` to set up isolated workspaces
```

新(new_string):

```markdown
- `create-workspace` — ensures feature work runs in its own herdr workspace; invoked from `design-discussion` (workspace check) and before `execute-plan`
```

- [ ] **Step 3: Verify**

```sh
cd /Users/sakumatomoya/workspace/dotfiles
grep -q "^### Workspaces$" claude/CLAUDE.global.md && echo "SECTION OK"
! grep -q "using-git-worktrees" claude/CLAUDE.global.md && echo "REF CLEAN"
git diff --check && git status --short
```

Expected: `SECTION OK` / `REF CLEAN`。

- [ ] **Step 4: Commit**

```sh
git add claude/CLAUDE.global.md
git commit -m "$(cat <<'EOF'
CLAUDE.global.md: herdr workspace 方針を明文化

feature 作業は herdr workspace(~/.herdr/worktrees の worktree)上の
セッションで design-discussion から finish-branch まで貫通する契約を
Workspaces 節として追加。Claude が herdr コマンドを実行し、セッションの
開始と workspace の削除はエンジニアが行う。Claude 自発の worktree は
.claude/worktrees のまま対象外。

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
EOF
)"
```

---

## Final verification (after all tasks)

```sh
cd /Users/sakumatomoya/workspace/dotfiles
# 1. 旧名の残骸ゼロ(過去プランの履歴 docs/plans は除外)
grep -rn "using-git-worktrees" claude/ | grep -v "claude/docs/plans/" ; test $? -eq 1 && echo "SWEEP CLEAN"
# 2. 新スキルの存在と旧ディレクトリの不在
test -f claude/skills/create-workspace/SKILL.md && test ! -d claude/skills/using-git-worktrees && echo "RENAME OK"
# 3. 参照カウント(execute-plan:4 / agent-teams:2 / design-discussion:1+ / CLAUDE.global.md:2)
grep -rc "create-workspace" claude/skills/execute-plan/SKILL.md claude/skills/agent-teams-driven-development/SKILL.md claude/skills/design-discussion/SKILL.md claude/CLAUDE.global.md
# 4. install.sh 構文チェック(変更していないことの確認を兼ねる)
bash -n claude/install.sh && echo "INSTALL SH OK"
# 5. コミット数
git log --oneline main..HEAD | wc -l   # Expected: 5
```

Expected: `SWEEP CLEAN` / `RENAME OK` / カウント 4, 2, ≥1, 2 / `INSTALL SH OK` / 5 commits。

**配布(マージ後):** main へのマージ後、main checkout で `bash claude/install.sh` を実行して `~/.claude/` に反映する。`rsync -a --delete` により `~/.claude/skills/using-git-worktrees/` は自動削除される。feature ブランチからは実行しない(未マージのスキルが配布されるため)。

## Post-/review iteration

Reserved for fix tasks appended by Claude Code after `/review` produces actionable items. Empty until `/review` runs.

(See CLAUDE.md "Core Flow" for the autonomous review feedback loop.)

## Push and PR

```sh
git push -u origin feat/herdr-workspace-policy
gh pr create --base main --title "AE ワークフローを herdr workspace 前提に移行: create-workspace スキル導入" --body "..."
```

PR 説明は PRDoc 形式(概要/利用側への影響/設計判断/変更内容/テスト/スコープ外/参考資料)。設計判断には design-discussion の決定(全フェーズ worktree 貫通、Claude が herdr コマンドを実行、セッション開始と削除はエンジニア)と Task 4 の追加発見を記載。

## Out of scope

- **memory 分断問題** — worktree セッションは別プロジェクト扱いになり本体の auto-memory が載らない。保留(挙動観察中)。候補策と再訪トリガーは memory `herdr-worktree-memory-deferral` に記録済み。
- **workspace / worktree / ブランチの削除の自動化** — エンジニアの手動操作(`herdr worktree remove`)。
- **herdr が無い環境の本格フォールバック** — 最小フォールバック(質問して停止)のみ。エンジニアの全マシンに herdr があるため。
- **過去プラン(`claude/docs/plans/*.md`)内の旧スキル名** — 履歴のため触らない。
- **`claude/skills/create-plan/example-plan.md`** — "worktree" の一般語のみで旧スキル名への参照なし。触らない。

## Alternative Solutions Considered

- **plan 承認境界でのセッション分割**(design/plan は本体 checkout、execute 以降を worktree セッション): plan だけがハンドオフになり設計議論のニュアンスが落ちる。**却下** — design-discussion から worktree 貫通なら会話の連続性が保たれる。
- **`herdr agent start` による新セッション自動起動+トピック種付け**: 手数最小だが、セッション開始の主導権が Claude に移る。**却下** — フェーズゲートをエンジニアが握る設計と不整合。
- **恒久規範の受け皿を CLAUDE.md に一本化**(auto-memory 退役): feature ブランチに CLAUDE.md 変更が乗るノイズ問題。**保留** — memory 問題ごと挙動観察へ(Out of scope 参照)。
- **旧ディレクトリ選択ロジック(.worktrees 探索 → CLAUDE.md grep → 質問)の温存**: 主環境が herdr に固定された以上、例外環境のための分岐は保守コストの方が高い。**却下**(YAGNI)。
