# Fable 蒸留第 2 弾 Implementation Plan

> **Execution:** エンジニア承認済みの逸脱(2026-07-08 /design-discussion): 本プランは agent-teams に dispatch せず、**Claude Code がメインセッションで直接編集**する。A/B テストは実施しない(エンジニア判断)。Steps は checkbox (`- [ ]`) で追跡する。

**Goal:** /review の finder 段 recall 修正(§2)、Fable 公式規範のチームメイトプロンプト・CLAUDE.global.md への移植(§3)、過剰指示の保守的削減(§4)により、Opus 系モデルでの自律ループ品質を公式ガイダンス準拠の形に引き上げる。

**Architecture:** 3 系統の独立した Markdown 編集。§2 は「自己フィルタを finder(4 ペルソナ)から integrator へ移動+`confidence` フィールド導入」(Task 1)。§3 は CLAUDE.global.md へ 2 節追加(Task 2)とチームメイトプロンプト 3 本への規範移植(Task 3)。§4 は design-discussion の徹底強制の軟化(Task 4)と verify の同一規則反復の圧縮(Task 5)。各タスクは独立コミットで、失敗時は revert 可能。

**Tech Stack:** Markdown(skills / agents / CLAUDE.global.md)。テストフレームワークなし — 検証は構造チェック(grep / wc / bash -n)。

**Working directory:** `/Users/sakumatomoya/workspace/dotfiles/claude`(全コマンドをここから実行)。
**Branch:** `fable-distillation-round2`。
**Baseline before Task 1:** `git status` clean / `diff ~/.claude/CLAUDE.md CLAUDE.global.md` が無差分 / `wc -l CLAUDE.global.md` = 163 / `bash -n install.sh` exit 0。

**Per-task verification command**(各コミット前に必須):
```sh
bash -n install.sh
```
(install.sh の誤編集ガード。加えて各タスク固有の grep / wc チェックがある。)

**Discipline 共通注記:** 全タスクは Markdown 設定の編集であり通常のテストコードを書けない。第 1 弾の A/B に相当する検証はエンジニア判断(2026-07-08)により**実施しない**。代替は (a) 構造チェック(必須文言の存在/不在を grep で確認)、(b) プランレビューでの文面承認、(c) 日常運用での挙動観測。

**設計判断の出典:** 2026-07-08 の /design-discussion(確定 6 項目)。根拠となる公式ドキュメント: [Prompting Claude Opus 4.8](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-opus-4-8)(Code review harnesses / literal instruction following)、[Prompting Claude Fable 5](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-fable-5)(蒸留スニペット群)、[Prompting best practices](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/claude-4-best-practices)(aggressive language の抑制)。

---

### Task 1: §2 — /review finder の自己フィルタを integrator へ移動+confidence 導入

**Why:** Opus 4.8 公式ガイドが名指しするレビューハーネスの recall アンチパターン(「be conservative」系指示により、発見済みのバグを報告段階で捨てる)に、現行の「再現を構成できなければ `findings: []`」ルールが該当する。公式推奨は「finder 段は網羅+confidence 添付、フィルタは別段」であり、フィルタ段(adversarial-integrator)は既存。

**Behavior change:** yes(/review の finder 出力が「網羅+confidence」に変わり、フィルタ責務が integrator に一元化される)
**Discipline:** 直接編集+構造チェック(共通注記参照)。

**Files:**
- Modify: `agents/adversarial-robustness-reviewer.md`(Reasoning Depth / Stance / Output Schema)
- Modify: `agents/adversarial-api-reviewer.md`(同上)
- Modify: `agents/adversarial-performance-reviewer.md`(同上)
- Modify: `agents/adversarial-tests-reviewer.md`(同上)
- Modify: `skills/review/SKILL.md`(adversarial-layer requirements / YAML schema / Red Flags)
- Modify: `agents/adversarial-integrator.md`(Input / Processing Step 4)

### Steps

- [ ] **Step 1: 4 ペルソナの Reasoning Depth を修正**

各ファイルの Reasoning Depth 末尾の禁止文を以下のとおり置換する。

`agents/adversarial-robustness-reviewer.md` — 旧:
```
**Speculative "this might be unsafe" without a constructed reproduction is forbidden.**
```
新:
```
Attempt a concrete reproduction for every hypothesis. If you cannot fully construct one, report the finding anyway with `confidence: low` and state what is missing — do not silently drop it.
```

`agents/adversarial-api-reviewer.md` — 旧:
```
**Speculative concerns without a constructed misuse are forbidden.**
```
新:
```
Attempt a concrete misuse for every hypothesis. If you cannot fully construct one, report the finding anyway with `confidence: low` and state what is missing — do not silently drop it.
```

`agents/adversarial-performance-reviewer.md` — 旧:
```
**Micro-optimizations without an execution-frequency argument are forbidden.**
```
新(マイクロ最適化の除外はスコープ規則として維持し、頻度論証が不完全な場合の報告経路を追加):
```
Micro-optimizations with no cost argument at all are out of scope. If the cost is real but you could not fully trace the execution frequency, report the finding with `confidence: low` and state the missing link in the caller chain — do not silently drop it.
```

`agents/adversarial-tests-reviewer.md` — 旧:
```
**Speculative "could be better" without a concrete passthrough is forbidden.**
```
新:
```
Attempt a concrete passthrough for every hypothesis. If you cannot fully construct one, report the finding anyway with `confidence: low` and state what is missing — do not silently drop it.
```

- [ ] **Step 2: 4 ペルソナの Stance を置換**

`agents/adversarial-robustness-reviewer.md` — 旧:
```
You are an adversarial reviewer. For this diff, try to construct exactly one input or scenario that makes it terminate unexpectedly / produce unhandled errors / exhibit undefined behavior. State the failure scenario as a hypothesis with concrete reproduction. **If you cannot construct one, return `findings: []` with `considered:` populated.** "Just in case" findings are forbidden.
```
新:
```
You are an adversarial reviewer. For this diff, try to construct inputs or scenarios that make it terminate unexpectedly / produce unhandled errors / exhibit undefined behavior. State each failure scenario as a hypothesis with a reproduction. Report every genuine concern you find, including ones you are uncertain about — do not filter for importance or confidence; the integrator filters downstream. Fabricating evidence is forbidden; reporting honest uncertainty as `confidence: low` is not. If a genuine hunt surfaces nothing, return `findings: []` with `considered:` populated.
```

`agents/adversarial-api-reviewer.md` — 旧:
```
You are an adversarial reviewer. For this API surface, construct exactly one realistic misuse: a way a competent consumer would naturally write code that misbehaves, given this API. The misuse must compile and look idiomatic. **If you cannot construct one, return `findings: []` with `considered:` populated.** Vague "could be confusing" without a concrete misuse pattern is forbidden.
```
新:
```
You are an adversarial reviewer. For this API surface, construct realistic misuses: ways a competent consumer would naturally write code that misbehaves, given this API. A misuse should compile and look idiomatic. Report every genuine concern you find, including ones you are uncertain about — do not filter for importance or confidence; the integrator filters downstream. Fabricating evidence is forbidden; reporting honest uncertainty as `confidence: low` is not. If a genuine hunt surfaces nothing, return `findings: []` with `considered:` populated.
```

`agents/adversarial-performance-reviewer.md` — 旧:
```
You are an adversarial reviewer. For this diff, identify code paths that (a) carry measurable cost and (b) execute at "N times" or higher frequency. State the execution frequency argument (caller chain, input-size relationship) in your hypothesis. **If you cannot identify such a path, return `findings: []` with `considered:` populated.** "Just-in-case optimization" is forbidden.
```
新:
```
You are an adversarial reviewer. For this diff, identify code paths that (a) carry measurable cost and (b) execute at "N times" or higher frequency. State the execution frequency argument (caller chain, input-size relationship) in your hypothesis. Report every genuine concern you find, including ones whose frequency argument is incomplete (`confidence: low`) — do not filter for importance or confidence; the integrator filters downstream. Micro-optimizations with no cost argument at all remain out of scope. If a genuine hunt surfaces nothing, return `findings: []` with `considered:` populated.
```

`agents/adversarial-tests-reviewer.md` — 旧:
```
You are an adversarial reviewer. For each test in this diff, try to construct: (a) a bug in the implementation that this test does not catch, (b) a way to replace the implementation with a no-op / identity that still passes this test. State the passthrough pattern concretely. **If you cannot construct one, return `findings: []` with `considered:` populated.** Vague "test could be stronger" without a concrete passthrough is forbidden.
```
新:
```
You are an adversarial reviewer. For each test in this diff, try to construct: (a) a bug in the implementation that this test does not catch, (b) a way to replace the implementation with a no-op / identity that still passes this test. State the passthrough pattern concretely. Report every genuine concern you find, including ones you are uncertain about — do not filter for importance or confidence; the integrator filters downstream. Fabricating evidence is forbidden; reporting honest uncertainty as `confidence: low` is not. If a genuine hunt surfaces nothing, return `findings: []` with `considered:` populated.
```

- [ ] **Step 3: 4 ペルソナの Output Schema に confidence を追加**

4 ファイル共通: Required fields リストの `- `severity_suggestion` — Critical / Important / Minor` の行の直後に以下を挿入する。

```
- `confidence` — high / medium / low: how certain you are the finding is real and reachable (low = the reproduction/argument could not be fully constructed)
```

- [ ] **Step 4: skills/review/SKILL.md を修正(3 箇所)**

(a) Adversarial-layer requirements の第 4 ビュレット — 旧:
```
- Return `findings: []` (with a `considered:` list of what was examined) when no genuine concerns were found. **Null-finding is acceptable** — speculative or "just in case" findings are forbidden
```
新:
```
- Report every genuine concern found, including uncertain ones, each carrying a `confidence` field. The finder stage optimizes for **coverage**; importance/confidence filtering happens downstream in the integrator (Step 2.5). Fabricated evidence is forbidden; honest uncertainty (`confidence: low`) is not. Return `findings: []` (with a `considered:` list) only when a genuine hunt surfaces nothing
```

(b) Adversarial Output Schema の YAML — `severity_suggestion: Critical | Important | Minor` の行の直後に挿入:
```
    confidence: high | medium | low  # low = 再現/根拠を完全には構成できなかった
```

(c) Red Flags の行 — 旧:
```
| Adversarial persona inventing speculative findings to "find something" | Null-finding is acceptable. Return `findings: []` with `considered:` when no genuine concern with concrete reproduction can be constructed. |
```
新(2 行に分割):
```
| Adversarial persona self-filtering findings it judges uncertain or low-severity | Report them with `confidence` and severity; the integrator filters downstream. Finder-stage self-filtering silently drops real bugs (recall loss). |
| Adversarial persona fabricating evidence or reproductions to "find something" | Evidence must come from code actually read. Honest uncertainty is reported as `confidence: low`, never dressed up as certainty. |
```

- [ ] **Step 5: agents/adversarial-integrator.md を修正(2 箇所)**

(a) Input の第 1 ビュレット — 旧:
```
- 4 YAML finding sets, one from each adversarial reviewer
```
新:
```
- 4 YAML finding sets, one from each adversarial reviewer (each finding carries a `confidence` field: high / medium / low)
```

(b) Processing Step 4 — 旧:
```
### 4. Evidence verification

For each finding, verify that `reproduction` is concrete (specific input, specific misuse pattern, specific execution path) rather than abstract speculation. Demote findings with weak reproduction one severity level (Critical → Important, Important → Minor). Drop Minor findings whose reproduction is purely speculative.
```
新:
```
### 4. Evidence verification (the filtering point)

The personas are instructed NOT to self-filter — uncertain findings arrive here by design, and this step is where speculative findings die. For each finding, verify that `reproduction` is concrete (specific input, specific misuse pattern, specific execution path) rather than abstract speculation, and read its `confidence`:

- Demote one severity level (Critical → Important, Important → Minor) when the reproduction is weak OR `confidence` is low.
- Drop the finding when `confidence` is low AND the reproduction is abstract AND the post-demotion severity is Minor.
- Never drop a finding solely for low `confidence` when its reproduction is concrete — low confidence with concrete evidence is exactly what the coverage design exists to surface.
```

- [ ] **Step 6: Verify(構造チェック)**

```sh
bash -n install.sh
grep -c 'confidence' agents/adversarial-robustness-reviewer.md agents/adversarial-api-reviewer.md agents/adversarial-performance-reviewer.md agents/adversarial-tests-reviewer.md
grep -c 'confidence' skills/review/SKILL.md agents/adversarial-integrator.md
grep -rn 'return `findings: \[\]` with `considered:` populated.\*\*' agents/ ; echo "forbidden-fallback: $?"
grep -c 'Null-finding is acceptable' skills/review/SKILL.md || true
grep -c 'the filtering point' agents/adversarial-integrator.md
```

Expected: 各ペルソナで `confidence` が 3 以上 / SKILL.md・integrator で 2 以上 / 「If you cannot construct one, return...」型の旧文言が agents/ に 0 件(grep が exit 1、`forbidden-fallback: 1`)/ `Null-finding is acceptable` が 0(grep exit 1 で `|| true` により継続)/ integrator に `the filtering point` が 1。

- [ ] **Step 7: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
Update: /review finder の自己フィルタを integrator に移動し recall を回復

Opus 4.8 公式ガイドが警告するレビューハーネスの recall アンチパターン
(finder 段の自己フィルタ)を解消する。adversarial ペルソナ 4 体は
confidence 付きで網羅報告し、投機的発見のフィルタは既存の
adversarial-integrator Step 4 に一元化。証拠の捏造禁止は finder に残す。

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
EOF
)"
```

---

### Task 2: §3a — CLAUDE.global.md にターン終端規律とアプローチ固定を追加

**Why:** Fable 公式ページの autonomous-pipeline スニペット(「最終段落が計画・約束なら今やる」)と overthinking 抑制スニペット(「choose an approach and commit」)は、第 1 弾 A/B の残課題(結論先出し 3/5 止まり・宣言して止まる挙動)への直接の手当て。行数予算は 2026-07-08 の design-discussion で ≤175 行に緩和済み。

**Behavior change:** yes(リードセッションのターン終端・意思決定挙動)
**Discipline:** 直接編集+構造チェック(共通注記参照)。

**Files:**
- Modify: `CLAUDE.global.md`(「## Role and Autonomy」セクション末尾に 2 小節追加)

### Steps

- [ ] **Step 1: 2 小節を挿入**

`CLAUDE.global.md` の Escalation Rule 末尾の段落:
```
When escalating, present what was tried, what failed, and recommend the engineer take over implementation if appropriate.
```
の直後(`## Reporting Completion — Evidence Before Claims` の前)に、以下を挿入する:

```markdown
### Turn-End Discipline

Before ending a turn, check the last paragraph of the reply. If it is a plan, a question the codebase can answer, or a promise about work not yet done ("I'll…", "let me know when…"), do that work now with tool calls instead of ending the turn. The exceptions are the stops this document itself mandates — gated phase transitions, escalations, and questions only the engineer can answer: there, ask and end the turn, rather than ending on a promise.

### Commit to the Approach

When weighing approaches during execution, choose one and commit. Do not revisit a decision unless new information directly contradicts the reasoning behind it. If the chosen approach fails, course-correct at that point — do not hedge across multiple approaches at once.
```

- [ ] **Step 2: Verify**

```sh
bash -n install.sh
wc -l CLAUDE.global.md
grep -c '^### Turn-End Discipline' CLAUDE.global.md
grep -c '^### Commit to the Approach' CLAUDE.global.md
grep -c 'ending on a promise' CLAUDE.global.md
```

Expected: `wc -l` ≤ 175(見込み 173)/ 各見出し 1 回 / `ending on a promise` 1 回。

- [ ] **Step 3: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
Update: CLAUDE.md にターン終端規律とアプローチ固定を追加(蒸留第2弾)

Fable 公式プロンプトガイドの autonomous-pipeline / overthinking 抑制
スニペットを Core Flow のゲート遷移と両立する条件付きで移植。
第 1 弾 A/B の残課題(宣言して止まる・結論先出しの不徹底)への手当て。
行数予算は design-discussion (2026-07-08) で ≤175 行に緩和済み。

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
EOF
)"
```

---

### Task 3: §3b+c — チームメイトプロンプト 3 本への規範移植

**Why:** 第 1 弾はリードのみを改善し、実作業を担う implementer には「聞く前に調べる」も報告の証拠監査もない(Evidence Before Claims の連鎖の根元が抜けている)。タスク単位レビュアー 2 体は下流フィルタなしで fix ループを直接駆動するため、/review と逆に precision 側の縛りが要る。

**Behavior change:** yes(チームメイトの質問・報告・レビュー挙動)
**Discipline:** 直接編集+構造チェック(共通注記参照)。

**Files:**
- Modify: `skills/agent-teams-driven-development/implementer-prompt.md`(Discipline 項目 1 / Report Format)
- Modify: `skills/agent-teams-driven-development/spec-reviewer-prompt.md`(Your Job 末尾)
- Modify: `skills/agent-teams-driven-development/code-quality-reviewer-prompt.md`(Your Job 末尾)

### Steps

- [ ] **Step 1: implementer の「聞く前に調べる」置換**

`implementer-prompt.md` — 旧:
```
1. **Before You Begin**: If you have questions about requirements, approach, dependencies, or anything unclear, ask them via SendMessage to the lead before starting work. Don't guess.
```
新:
```
1. **Investigate, then ask**: Read the files named in the task and their surrounding code first. Never ask the lead a question the plan or the codebase can already answer. If genuine ambiguity remains (requirements, approach, dependencies), ask via SendMessage before starting work — all remaining questions in one batch. Don't guess.
```

- [ ] **Step 2: implementer の報告に証拠監査を追加**

`implementer-prompt.md` の Report Format のビュレットリスト:
```
- **Concerns** (if any)
```
の直後、`Use DONE_WITH_CONCERNS if you completed but have doubts about correctness.` の前に挿入:

```
Before reporting, audit each claim against a tool result from this run (test output, build exit code, diff). Only report work you can point to evidence for; if something is not yet verified, say so explicitly under Concerns.
```

- [ ] **Step 3: spec-reviewer に precision 側の縛りを追加**

`spec-reviewer-prompt.md` の Your Job 末尾:
```
Verify by reading code, not by trusting report.
```
の直後に挿入:

```
Approval is a valid outcome. Your report gates the task directly — there is no downstream filter — so report only issues you verified against the diff (file:line), not speculative concerns. (The /review adversarial personas work differently: their findings pass through an integrator filter, so they optimize for coverage. You optimize for precision.)
```

- [ ] **Step 4: code-quality-reviewer に同趣旨を追加**

`code-quality-reviewer-prompt.md` の Discipline ブロック末尾:
```
- Did they follow existing patterns?
```
の直後(`## Sending Messages` の前)に挿入:

```
Approval is a valid outcome. Do not manufacture issues to justify the review — your report gates the task directly, with no downstream filter, so report only issues you verified against the diff (file:line). Flag what affects correctness, code organization, testing, or maintainability; style preferences are not issues.
```

- [ ] **Step 5: Verify**

```sh
bash -n install.sh
grep -c 'Investigate, then ask' skills/agent-teams-driven-development/implementer-prompt.md
grep -c 'audit each claim against a tool result' skills/agent-teams-driven-development/implementer-prompt.md
grep -c 'Approval is a valid outcome' skills/agent-teams-driven-development/spec-reviewer-prompt.md skills/agent-teams-driven-development/code-quality-reviewer-prompt.md
grep -c 'Before You Begin' skills/agent-teams-driven-development/implementer-prompt.md || true
```

Expected: 新文言が各 1 回 / `Before You Begin` が 0(grep exit 1)。

- [ ] **Step 6: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
Update: チームメイトプロンプトに蒸留規範を移植(蒸留第2弾)

implementer に「聞く前に調べる」と報告の証拠監査(Fable 公式で捏造
ステータス報告をほぼ排除と実証済みの文面)を追加。タスク単位レビュアー
2 体には「Approval is a valid outcome」の precision 側の縛りを追加 —
/review の coverage 方針と逆である理由(下流フィルタの有無)も明記。

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
EOF
)"
```

---

### Task 4: §4a — design-discussion の徹底強制を意図保存で軟化

**Why:** 「Grill ... relentlessly / Do not exit grilling」は旧モデルの怠慢対策由来の徹底強制で、指示に忠実な Opus 4.8 では質問過多を誘発し(公式: aggressive language は overtrigger を招く)、第 1 弾で入れた「Ask the one question that matters most」と方向が衝突する。クリティカルパス全決定のカバレッジという意図は保存する。責務ゲート(役割境界・ルーティング・プロトタイピング規則)は不変。

**Behavior change:** yes(design-discussion 中の質問カデンツ)
**Discipline:** 直接編集+構造チェック(共通注記参照)。

**Files:**
- Modify: `skills/design-discussion/SKILL.md`(5 箇所)

### Steps

- [ ] **Step 1: Operating Procedure 前文**

旧:
```
**Operating Procedure (mandatory).** The engineer should never have to
prompt for "ultrathink" or "grill harder" — both are baked into this
skill.
```
新:
```
**Operating Procedure (mandatory).** The engineer should never have to
prompt for "ultrathink" or deeper questioning — both are baked into this
skill.
```

- [ ] **Step 2: Process 手順 3 の置換**

旧:
```
3. **Grill the engineer through the decision tree relentlessly.** One
   question at a time; multiple-choice or recommend-an-answer formats
   preferred. Walk branch by branch, surfacing dependencies between
   decisions. Cover both **problem-space decisions** (what must be
   required, guaranteed, or exposed — consistency, failure tolerance,
   integration boundaries, performance budgets) and **solution-space
   decisions** (which architecture, which structural alternative).
   Continue until the engineer and Claude Code share complete
   understanding of every critical-path decision. **Do not exit
   grilling on surface answers or perceived simplicity.** Non-blocking
   branches may be deferred with an explicit note.
```
新:
```
3. **Walk the engineer through the decision tree.** One question at a
   time — the single highest-leverage question next; multiple-choice or
   recommend-an-answer formats preferred. Walk branch by branch,
   surfacing dependencies between decisions. Cover both **problem-space
   decisions** (what must be required, guaranteed, or exposed —
   consistency, failure tolerance, integration boundaries, performance
   budgets) and **solution-space decisions** (which architecture, which
   structural alternative). Continue until every critical-path decision
   is resolved — do not stop at a surface answer when a deeper branch
   materially changes the design. Non-blocking branches may be deferred
   with an explicit note.
```

- [ ] **Step 3: Process 結び**

旧:
```
Scale the depth to the work, but **do not skip ultrathink and do not
exit grilling early**.
```
新:
```
Scale the depth to the work; investigation-first and ultrathink remain
mandatory at every depth.
```

- [ ] **Step 4: Role ビュレット**

旧:
```
- Grill through the decision tree with recommended answers and trade-offs to surface assumptions, constraints, and non-obvious dependencies
```
新:
```
- Walk the decision tree with recommended answers and trade-offs to surface assumptions, constraints, and non-obvious dependencies
```

- [ ] **Step 5: Red Flags 行**

旧:
```
| Claude Code waits for the engineer to prompt "ultrathink" or "grill harder" | Both are mandatory by default per Operating Procedure. Apply them without prompting. |
```
新:
```
| Claude Code waits for the engineer to prompt "ultrathink" or deeper investigation | Both are defaults per Operating Procedure. Apply them without prompting. |
```

- [ ] **Step 6: Verify**

```sh
bash -n install.sh
grep -ci 'grill' skills/design-discussion/SKILL.md || true
grep -c 'highest-leverage question' skills/design-discussion/SKILL.md
grep -c 'ultrathink' skills/design-discussion/SKILL.md
grep -c 'Walk the decision tree' skills/design-discussion/SKILL.md
```

Expected: `grill` 0 件(grep exit 1)/ `highest-leverage question` 1 / `ultrathink` は 3 以上(mandate 維持の証拠)/ `Walk the decision tree` は Key Principles 行と合わせ 1 以上。

- [ ] **Step 7: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
Update: design-discussion の徹底強制を意図保存で軟化(蒸留第2弾)

「grill relentlessly / do not exit grilling」は指示に忠実な Opus 4.8 で
質問過多を誘発し、CLAUDE.md の「急所の一問」と衝突する(公式:
aggressive language は overtrigger を招く)。クリティカルパス全決定の
カバレッジ要件と ultrathink の mandate は保持。責務ゲートは不変。

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
EOF
)"
```

---

### Task 5: §4b — verify の同一規則反復を圧縮

**Why:** verify/SKILL.md は Iron Law を実質 6 箇所(Core Principle / Iron Law / Gate Function / Red Flags / Rationalization / Bottom Line)で反復しており、公式ガイダンスではこの重複は強調ではなく希釈として働く(重要行が埋もれる)。A/B なしのため軽〜中程度に留め、ゲート意味論の削除ゼロを検証条件とする。

**Behavior change:** no(意味論保存の圧縮 — 全ゲート・全手順・全表の固有内容は維持)
**Discipline:** 直接編集+構造チェック(共通注記参照)。

**Files:**
- Modify: `skills/verify/SKILL.md`(3 箇所の削除・統合。181 行 → 約 164 行)

### Steps

- [ ] **Step 1: Core Principle 節を The Iron Law に統合**

旧(Core Principle 節全体と Iron Law 節の導入):
`````
## Core Principle

**Evidence before claims, always.**

Claiming work is complete without fresh verification is dishonesty, not efficiency.

**Violating the letter of this rule is violating the spirit of this rule.**

## The Iron Law

```
NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE
```

If you haven't run the verification command in this session, you cannot claim it passes.
`````
新:
`````
## The Iron Law

```
NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE
```

If you haven't run the verification command in this session, you cannot claim it passes. Claiming completion without fresh verification is dishonesty, not efficiency — and violating the letter of this rule is violating its spirit.
`````

- [ ] **Step 2: Red Flags - STOP の重複 2 項を削除**

以下の 2 行を削除する(いずれも Rationalization Prevention 表の「"Just this once"」「"I'm tired"」と同一規則):
```
- Thinking "just this once"
- Tired and wanting work over
```

- [ ] **Step 3: The Bottom Line 節を削除**

以下の節全体を削除する(意味論は The Iron Law と The Gate Function に既在):
```
## The Bottom Line

**No shortcuts for verification.**

Run the command. Read the output. THEN claim the result.

This is non-negotiable.
```

- [ ] **Step 4: Verify**

```sh
bash -n install.sh
wc -l skills/verify/SKILL.md
grep -c 'NO COMPLETION CLAIMS WITHOUT FRESH VERIFICATION EVIDENCE' skills/verify/SKILL.md
grep -c 'The Bottom Line' skills/verify/SKILL.md || true
grep -ci 'just this once' skills/verify/SKILL.md
grep -c 'Skip any step = lying' skills/verify/SKILL.md
grep -c '## The Gate Function' skills/verify/SKILL.md
grep -c '## When To Apply' skills/verify/SKILL.md
```

Expected: 約 164 行(≤170)/ Iron Law 本文 1 回 / `The Bottom Line` 0(grep exit 1)/ `just this once` は 1 回のみ(Rationalization 表)/ Gate Function の「lying」文・Gate Function 節・When To Apply 節は各 1 回(ゲート意味論の維持の証拠)。

- [ ] **Step 5: Commit**

```sh
git add -A
git commit -m "$(cat <<'EOF'
Refactor: verify スキルの同一規則反復を圧縮(蒸留第2弾)

Iron Law の 6 箇所反復のうち純粋な重複(Core Principle / Bottom Line /
Red Flags 2 項)を正準記述に統合。公式ガイダンスの「重複は強調ではなく
希釈」に基づく。No behavior change: 全ゲート・手順・表の固有内容は維持。

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
EOF
)"
```

---

## Final verification (after all tasks)

```sh
cd /Users/sakumatomoya/workspace/dotfiles/claude
bash -n install.sh
wc -l CLAUDE.global.md skills/verify/SKILL.md skills/design-discussion/SKILL.md
grep -rn 'Null-finding is acceptable' skills/ agents/ || echo "OK: null-finding self-filter removed"
grep -rci 'grill' skills/design-discussion/SKILL.md || echo "OK: grill removed"
grep -c 'confidence' skills/review/SKILL.md
git log --oneline main..HEAD
```

Expected: `bash -n` exit 0 / CLAUDE.global.md ≤175・verify ≤170 / 旧文言 2 種が 0 件(OK 行が出る)/ review SKILL に confidence ≥2 / ブランチに 5 コミット。

**配布注記:** `./install.sh` は本ブランチでは実行しない。マージ後にエンジニア(または指示を受けた Claude Code)が実行して `~/.claude/` に反映する。

## Post-/review iteration

Reserved for fix tasks appended by Claude Code after `/review` produces actionable items. Empty until `/review` runs.

(See CLAUDE.md "Core Flow" for the autonomous review feedback loop.)

## Push and PR

```sh
git push -u origin fable-distillation-round2
gh pr create --base main --title "Fable蒸留第2弾: /review recall修正、チームメイト規範移植、過剰指示の削減" --body "..."
```

PR 説明は PRDoc 形式(概要/利用側への影響/設計判断/変更内容/テスト/スコープ外/参考資料)。「テスト」節には A/B 不実施(エンジニア判断)と構造チェック結果を明記。「参考資料」に公式ドキュメント 3 本(Prompting Claude Opus 4.8 / Prompting Claude Fable 5 / Prompting best practices)をリンク。

## Out of scope

- effort 設定(セッション/エージェントの effort 制御)— エンジニア判断で config 管理とし、本イテレーションから除外
- 規律の hooks 化(§5)— 実運用で advisory ルール違反が観測されたものから、別イテレーションで
- commit スキルの stale な Co-Authored-By 修正ほか housekeeping(初回調査の候補 1・5 以下)— 本スコープ外
- verify のより深い構造圧縮・残り 15 スキルの prescriptiveness 監査 — 挙動観察後の第 3 弾
- 検証レビュアー(design-alignment / scope / test-coverage)への confidence 導入 — integrator を通らないため対象外(YAGNI)
- A/B テストによる挙動検証 — エンジニア判断(2026-07-08)で不実施。日常運用で観測し、問題があれば次イテレーション

## Alternative Solutions Considered

- **第 1 弾型の A/B 検証(ヘッドレス Opus 4.8 × ルーブリック)**: 挙動差を採点して合否判定。**Rejected because**: エンジニア判断(2026-07-08)— プラン+直接編集で速度優先。検証は構造チェック+プランレビュー+日常観測に代替。
- **finder 段の投機的発見禁止の維持**: 現行の「再現を構成できなければ `findings: []`」を保つ。**Rejected because**: Opus 4.8 公式が名指しする recall アンチパターン。フィルタは下流(integrator)に既在で、ループ空転防止の意図は保たれる。
- **CLAUDE.global.md 追加分を同一ファイル内圧縮で相殺(≤165 維持)**: **Rejected because**: 第 1 弾でチューニング済みの文面を A/B の網なしで削るのは回帰リスクが高い。予算を ≤175 に緩和(design-discussion で承認)。
- **17 スキル全体の prescriptiveness 監査**: **Rejected because**: A/B なしの直接編集では blast radius が過大。矛盾が明確な design-discussion と重複密度最大の verify の 2 ファイルに限定し、残りは第 3 弾へ。
- **agent-teams による実行**: Core Flow 標準の実行機構。**Rejected because**: エンジニア指示(2026-07-08)により直接編集方式。対象が Markdown 設定でありテスト駆動の対象外、かつ全編集の exact な新旧テキストがプラン(本書)でレビュー可能。
