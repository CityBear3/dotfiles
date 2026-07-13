# Review ループ誤遷移バグ修正 Implementation Plan

> **Execution:** Use `/execute-plan` to dispatch this plan to agent-teams. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** review ループが fix 実行後に `/verify` → `/review` を経ずに `/finish-branch` へ直行する誤遷移を、スキル定義 3 箇所の修正で排除する。

**Architecture:** 3 つの独立した防御層を修正する。(1) 主因である `agent-teams-driven-development` の Integration セクションの自己矛盾(Completion は `/verify` 遷移と書くのに Required リストは `/finish-branch` を指す)を解消。(2) clean 判定の判定者を「fix 実行後の fresh な `/review` 実行のみ」と `/review` スキルと CLAUDE.global.md の両方(機構と契約)で明確化し、agent-teams 内部レビュアーの approve との語の衝突を断つ。(3) `/finish-branch` の Entry Conditions に自動遷移時の入口ガードを追加し、clean な `/review` の証拠がなければ `/verify` へ戻す。3 層は独立に機能する: 1 が誤誘導の源を消し、2 が誤読の余地を消し、3 が突破時の最終防衛線になる。

**Tech Stack:** Markdown(スキル定義・グローバル指示)のみ。コード変更なし。

**Working directory:** `/Users/sakumatomoya/.herdr/worktrees/dotfiles/fix-review-loop-transition`(全コマンドはリポジトリルートから実行)。
**Branch:** `fix-review-loop-transition`(herdr workspace w8、チェックアウト済み)。
**Baseline before Task 1:** `git status` clean。以下の 3 つの grep が現状テキストを返すこと(修正対象の存在確認):

```sh
grep -n '`/finish-branch` — after all tasks complete' claude/skills/agent-teams-driven-development/SKILL.md
grep -n 'when review reports no Must Fix / Should Improve' claude/CLAUDE.global.md
grep -n 'until `review` reports no remaining items' claude/CLAUDE.global.md
grep -n '`review` has been completed and the engineer is satisfied' claude/skills/finish-branch/SKILL.md
```

プラン承認後、Task 1 開始前にプランファイル自体をコミットする:

```sh
git add claude/docs/plans/2026-07-13-fix-review-loop-transition.md
git commit -m "docs: review ループ誤遷移バグ修正の実装プランを追加"
```

**Per-task verification command**(各コミット前に必須):

```sh
cd /Users/sakumatomoya/.herdr/worktrees/dotfiles/fix-review-loop-transition && git diff --check && git status --short
```

Expected: 空白エラーなし、変更ファイルがそのタスクの Files 欄と一致。加えて各タスク固有の grep 検証(各タスクの Verify ステップ参照)。

**編集上の注意:** 編集は dotfiles リポジトリ側のみ。`~/.claude/` は直接編集しない(反映は `claude/install.sh`、本プランのスコープ外)。SKILL.md / CLAUDE.global.md の本文は英語なので、追記テキストも英語で書く(設計判断の内容は下記の日本語 Why に、実テキストは各 Step のコードブロックに記載)。

---

### Task 1: agent-teams Integration セクションの自己矛盾解消

**Why:** 誤遷移の主因。Completion セクション(124 行目)は「全タスク完了後は `/execute-plan` → `/verify` 遷移へ直行」と書く一方、Integration セクションの Required リスト(198 行目)は `- /finish-branch — after all tasks complete` と書いており矛盾する。fix タスク完了時点でこの行が最も新しいスキル指示となり、`/finish-branch` 直行を正規ルートとして誘導する。Required リストを Completion 本文と整合させる。

**Behavior change:** yes(スキル挙動定義の変更。実行可能テストはなく、検証は grep)
**Discipline:** doc edit — ステップごとの grep 検証

**Files:**
- Modify: `claude/skills/agent-teams-driven-development/SKILL.md`(Integration セクション、195–198 行目付近)

### Steps

- [ ] **Step 1: Required リストの `/finish-branch` 行を `/verify` に差し替え**

`claude/skills/agent-teams-driven-development/SKILL.md` の Integration セクションで、次の 1 行を Edit で置換する。

置換前(old_string):

```markdown
- `/finish-branch` — after all tasks complete
```

置換後(new_string):

```markdown
- `/verify` — after all tasks complete (via the parent flow's `/execute-plan` → `/verify` transition; see Completion)
```

- [ ] **Step 2: grep 検証**

```sh
grep -n '`/finish-branch` — after all tasks complete' claude/skills/agent-teams-driven-development/SKILL.md; echo "exit=$?"
grep -n '`/verify` — after all tasks complete' claude/skills/agent-teams-driven-development/SKILL.md
```

Expected: 1 つ目は no match(exit=1)。2 つ目は Integration セクションの 1 行がヒット。

- [ ] **Step 3: Per-task verification command を実行**

```sh
cd /Users/sakumatomoya/.herdr/worktrees/dotfiles/fix-review-loop-transition && git diff --check && git status --short
```

Expected: 空白エラーなし。変更ファイルは `claude/skills/agent-teams-driven-development/SKILL.md` のみ。

- [ ] **Step 4: Commit**

```sh
git add claude/skills/agent-teams-driven-development/SKILL.md
git commit -m "$(cat <<'EOF'
agent-teams: Integration の /finish-branch 直行誘導を /verify に修正

Completion セクションは「全タスク完了後は /execute-plan → /verify 遷移」
と定めるのに、Integration の Required リストは /finish-branch を指して
いた。fix タスク完了時にこの行が最新のスキル指示となり、/verify →
/review の再実行を飛ばして /finish-branch へ直行する誤遷移を誘導して
いた(review ループ中断バグの主因)。Required リストを Completion と
整合させる。
EOF
)"
```

---

### Task 2: clean 判定の明確化(/review Step 4 + CLAUDE.global.md)

**Why:** 増幅要因「review という語の衝突」への対処。クリーンレビュー時の自動遷移条件の "review" が、`/review` スキルではなく agent-teams 内部レビュアー(spec-reviewer / code-quality-reviewer)の approve と混同される。`/review` Step 4 の "All items are resolved" は fix 実行+チームレビュアー approve の時点で字義上満たされたと誤読でき、resolved の判定者が「次の fresh な `/review` 実行」であることが明記されていない。機構(`/review` スキル)と契約(CLAUDE.global.md)の両レベルで判定者を一意にする。CLAUDE.global.md 側は自動遷移 2(129 行目)に加え、Triage 契約の段落(132 行目)の "until `review` reports no remaining items" にも同種の曖昧さがあるため両方を明確化する(walkthrough 時のエンジニア判断で追加)。

**Behavior change:** yes(スキル挙動定義+グローバル契約の変更。検証は grep)
**Discipline:** doc edit — ステップごとの grep 検証

**Files:**
- Modify: `claude/skills/review/SKILL.md`(Step 4 の "The engineer is surfaced only when" 2 つ目の箇条書き、185 行目付近)
- Modify: `claude/CLAUDE.global.md`(automatic transitions リストの 2 番〔129 行目付近〕と Triage 契約段落〔132 行目付近〕)

### Steps

- [ ] **Step 1: /review Step 4 の clean 判定を明確化**

`claude/skills/review/SKILL.md` の Step 4、"The engineer is surfaced only when:" の 2 つ目の箇条書きを Edit で置換する。

置換前(old_string):

```markdown
- All items are resolved (any combination of push back / fix / no items at all) and the report has no remaining Must Fix / Should Improve. In this case, present the final clean report with the triage summary and **transition to `/finish-branch` automatically**
```

置換後(new_string):

```markdown
- All items are resolved and the report has no remaining Must Fix / Should Improve. An item counts as **resolved** only when it was pushed back, or when a subsequent fresh `/review` run no longer reports it — executing a fix does NOT resolve an item. A report from which even one Fix task was executed is therefore never clean: the Fix path always re-enters the loop (`/execute-plan` → `/verify` → `/review`), and only that next fresh `/review` run — never the agent-teams internal reviewers' (spec-reviewer / code-quality-reviewer) approval — renders the clean verdict. When the clean verdict holds, present the final clean report with the triage summary and **transition to `/finish-branch` automatically**
```

(箇条書きの後続部分 "— on a clean review this transition is **NOT** gated…" 以降は変更しない。old_string / new_string は箇条書き前半のみを対象とする部分置換。)

- [ ] **Step 2: CLAUDE.global.md の自動遷移 2 と Triage 契約段落を一語明確化**

`claude/CLAUDE.global.md` に対して 2 つの Edit を行う。

置換 1 — automatic transitions リスト 2 番。置換前(old_string):

```markdown
2. **Clean review → `/finish-branch`** — when review reports no Must Fix / Should Improve.
```

置換後(new_string):

```markdown
2. **Clean review → `/finish-branch`** — when `/review` (the skill) reports no Must Fix / Should Improve.
```

置換 2 — Triage 契約段落(132 行目付近)。置換前(old_string):

```markdown
and the loop continues until `review` reports no remaining items
```

置換後(new_string):

```markdown
and the loop continues until `/review` (the skill) reports no remaining items
```

- [ ] **Step 3: grep 検証**

```sh
grep -n 'any combination of push back / fix / no items at all' claude/skills/review/SKILL.md; echo "exit=$?"
grep -n 'fresh `/review` run no longer reports it' claude/skills/review/SKILL.md
grep -n 'never the agent-teams internal reviewers' claude/skills/review/SKILL.md
grep -n 'when `/review` (the skill) reports no Must Fix / Should Improve' claude/CLAUDE.global.md
grep -n 'until `/review` (the skill) reports no remaining items' claude/CLAUDE.global.md
grep -n 'when review reports no Must Fix / Should Improve' claude/CLAUDE.global.md; echo "exit=$?"
grep -n 'until `review` reports no remaining items' claude/CLAUDE.global.md; echo "exit=$?"
```

Expected: 1 つ目は no match(exit=1)。2〜5 つ目は各 1 行ヒット。6〜7 つ目は no match(exit=1)。

- [ ] **Step 4: Per-task verification command を実行**

```sh
cd /Users/sakumatomoya/.herdr/worktrees/dotfiles/fix-review-loop-transition && git diff --check && git status --short
```

Expected: 空白エラーなし。変更ファイルは `claude/skills/review/SKILL.md` と `claude/CLAUDE.global.md` の 2 つのみ。

- [ ] **Step 5: Commit**

```sh
git add claude/skills/review/SKILL.md claude/CLAUDE.global.md
git commit -m "$(cat <<'EOF'
review/global: clean 判定の判定者を fresh な /review 実行に限定

「review が Must Fix / Should Improve なしを報告 → finish-branch」の
"review" が agent-teams 内部レビュアーの approve と混同され、fix 実行
+チームレビュアー approve の時点で clean と誤読される余地があった。

- /review Step 4: resolved の定義を「push back 済み、または後続の
  fresh /review が報告しなくなった」に限定。Fix を 1 件でも実行した
  報告書は clean とみなさず、clean 判定は fix 後の fresh な /review
  のみが下すと明記。
- CLAUDE.global.md: 自動遷移 2 の "when review reports" を
  "when /review (the skill) reports" に、Triage 契約段落の
  "until review reports" を "until /review (the skill) reports" に
  明確化(契約レベルでも内部レビュアー approve との衝突を断つ)。
EOF
)"
```

---

### Task 3: finish-branch 入口ガードの追加

**Why:** 最終防衛線。現状の Entry Conditions は「review has been completed」とあるだけで検証可能な入口ガードがなく、Task 1・2 の誘導修正をすり抜けた誤遷移を止められない。自動遷移で入った場合に「最終コミット以降に clean な `/review` 完了の証拠がなければ `/verify` へ戻る」ガードを追加する。エンジニアが明示的に finish-branch を指示するパスは現状どおりガード対象外。

**Behavior change:** yes(スキル挙動定義の変更。検証は grep)
**Discipline:** doc edit — ステップごとの grep 検証

**Files:**
- Modify: `claude/skills/finish-branch/SKILL.md`(Entry Conditions セクション、17–20 行目付近)

### Steps

- [ ] **Step 1: Entry Conditions を置換しガードを追加**

`claude/skills/finish-branch/SKILL.md` の Entry Conditions セクション全体を Edit で置換する。

置換前(old_string):

```markdown
## Entry Conditions

- `review` has been completed and the engineer is satisfied with the results
- Or the engineer explicitly decides to finish the branch at any point
```

置換後(new_string):

```markdown
## Entry Conditions

- `/review` (the skill) has run to completion and reported clean — no Must Fix / Should Improve
- Or the engineer explicitly decides to finish the branch at any point (the entry guard below does not apply)

**Entry guard (automatic transition only):** when entered via the Core Flow's clean-review automatic transition, verify there is evidence of a clean `/review` completion since the latest commit: the most recent `/review` report in this session shows zero Must Fix / Should Improve, and no commit has been made after that report. If fixes were committed after the last `/review` report — or no `/review` report exists — do not proceed: return to `/verify` (the loop continues `/verify` → `/review` → back here). Agent-teams internal reviewer approval (spec-reviewer / code-quality-reviewer) is NOT such evidence.
```

- [ ] **Step 2: grep 検証**

```sh
grep -n '`review` has been completed and the engineer is satisfied' claude/skills/finish-branch/SKILL.md; echo "exit=$?"
grep -n 'Entry guard (automatic transition only)' claude/skills/finish-branch/SKILL.md
grep -n 'return to `/verify`' claude/skills/finish-branch/SKILL.md
```

Expected: 1 つ目は no match(exit=1)。2〜3 つ目は各 1 行ヒット。

- [ ] **Step 3: Per-task verification command を実行**

```sh
cd /Users/sakumatomoya/.herdr/worktrees/dotfiles/fix-review-loop-transition && git diff --check && git status --short
```

Expected: 空白エラーなし。変更ファイルは `claude/skills/finish-branch/SKILL.md` のみ。

- [ ] **Step 4: Commit**

```sh
git add claude/skills/finish-branch/SKILL.md
git commit -m "$(cat <<'EOF'
finish-branch: 自動遷移時の入口ガードを追加

Entry Conditions は「review has been completed」とあるだけで検証可能な
ガードがなく、誘導修正をすり抜けた誤遷移を止められなかった。自動遷移で
入った場合、最終コミット以降に clean な /review 完了の証拠(Must Fix /
Should Improve ゼロの報告書、かつその後コミットなし)がなければ /verify
へ戻すガードを追加。agent-teams 内部レビュアーの approve は証拠と
みなさない。エンジニアが明示的に指示するパスはガード対象外。
EOF
)"
```

---

## Final verification (after all tasks)

```sh
cd /Users/sakumatomoya/.herdr/worktrees/dotfiles/fix-review-loop-transition
git diff --check && git status --short
# 旧テキストが全て消えていること(全て no match / exit=1 であること)
grep -n '`/finish-branch` — after all tasks complete' claude/skills/agent-teams-driven-development/SKILL.md; echo "exit=$?"
grep -n 'any combination of push back / fix / no items at all' claude/skills/review/SKILL.md; echo "exit=$?"
grep -rn 'when review reports no Must Fix' claude/CLAUDE.global.md; echo "exit=$?"
grep -n 'until `review` reports no remaining items' claude/CLAUDE.global.md; echo "exit=$?"
grep -n '`review` has been completed and the engineer is satisfied' claude/skills/finish-branch/SKILL.md; echo "exit=$?"
# 新テキストが全て存在すること(各 1 行ヒット)
grep -n '`/verify` — after all tasks complete' claude/skills/agent-teams-driven-development/SKILL.md
grep -n 'fresh `/review` run no longer reports it' claude/skills/review/SKILL.md
grep -n 'when `/review` (the skill) reports no Must Fix / Should Improve' claude/CLAUDE.global.md
grep -n 'until `/review` (the skill) reports no remaining items' claude/CLAUDE.global.md
grep -n 'Entry guard (automatic transition only)' claude/skills/finish-branch/SKILL.md
```

Expected: working tree clean(コミット済み)。旧テキスト 5 件は全て no match、新テキスト 5 件は各 1 行ヒット。コミットは計 4 つ(プランファイル + Task 1〜3)。

## Post-/review iteration

Reserved for fix tasks appended by Claude Code after `/review` produces actionable items. Empty until `/review` runs.

(See CLAUDE.md "Core Flow" for the autonomous review feedback loop.)

## Push and PR

```sh
git push -u origin fix-review-loop-transition
gh pr create --base main --title "review ループ誤遷移の修正: fix 後に /verify → /review を経ず finish-branch へ直行するバグ" --body "..."
```

PR 説明は PRDoc 形式(概要 / 利用側への影響 / 設計判断 / 変更内容 / テスト / スコープ外 / 参考資料)で書く。push と PR 作成はエンジニアの確認後に実行する。

## Out of scope

- `/review` の Fix パスへの「`/execute-plan` は Skill ツールで再 invoke せよ」の追記 — グローバル契約(CLAUDE.global.md「actually invoke the corresponding skill via the Skill tool」)に既にあり、スキル側への複製は SSOT 設計(契約=global / 機構=スキル)に反する(設計段階で棄却済み)。
- 増幅要因「強調の非対称性」への追加対処(fix 後ループ継続の反復記述の増強)— 修正 1〜3 で誤誘導の源・誤読の余地・突破時の防衛線が揃うため、これ以上の反復追加は過剰と判断。
- CLAUDE.global.md 自動遷移 1(Review feedback loop)の "review" の語の同様の明確化 — 遷移 1 は fix パス側の記述で誤読の主対象ではない。
- `claude/install.sh` の実行(`~/.claude/` への反映)— main へのマージ後にエンジニアが実施。

## Alternative Solutions Considered

- **/review Fix パスの Skill 再 invoke 明記**: `/review` の Fix パスに「`/execute-plan` は Skill ツールで再 invoke せよ」と追記する強化策。**Rejected because**: グローバル契約に既に存在し、スキル側への複製は SSOT 設計に反する。修正 1 が入ればショートカット発生時も誤誘導は解消される。
- **入口ガードをエンジニア明示パスにも適用**: `/finish-branch` のガードを全入口に適用する案。**Rejected because**: エンジニアの明示判断はワークフロー上の最上位権限であり、ガードで縛るのは Role and Autonomy の設計(エンジニアの判断が優先)に反する。
