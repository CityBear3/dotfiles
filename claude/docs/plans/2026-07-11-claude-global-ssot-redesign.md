# CLAUDE.global.md SSOT 再設計(方針 B)Implementation Plan

> **Execution:** エンジニア承認(2026-07-11)により **直接編集**(agent-teams ループなし)。検証は構造チェック+日常観測。Steps はチェックボックスで進捗管理。
> 本プランは同日付の旧プラン `2026-07-11-claude-global-fable-readjustment.md`(未コミット)を置換し、その編集 4 点を統合する。

**Goal:** CLAUDE.global.md を「global = 契約(フェーズ・ゲート・エスカレーション・自律境界)、スキル = 機構」の SSOT 構造に再設計し、code-architect 分析で確定した実装乖離 7 項目と、公式 Prompting Claude Fable 5 との差分(boundaries / 再接地 / 委譲 / ask 形式処方)を一括解消する。

**Architecture:** Task 1 が CLAUDE.global.md への 11 編集(SSOT 化+正確化+Fable 統合、171 → 167 行)。Task 2 が周辺 3 ファイルの外科修正(stale な起動元記述 2 件と、即 ask 問題の発生源である design-discussion の multiple-choice 優先 2 記述)。設計原則は公式ガイドの 3 原則 — brief instruction > 列挙 / 停止条件を書き形式を書かない / 旧モデル向け足場の撤去。

**Tech Stack:** Markdown(プロンプト設定)。テストコードなし。

**Working directory:** `/Users/sakumatomoya/workspace/dotfiles`(全コマンドをここから実行)。
**Branch:** `claude-global-ssot-redesign`。
**Baseline before Task 1:** `git status` クリーン、`wc -l claude/CLAUDE.global.md` = 171。

**Discipline 共通注記:** 本プランは Markdown 設定の編集であり通常のテストコードを書けない。検証は (a) 構造チェック(必須文言の存在/不在を grep で確認、行数予算 ≤175)、(b) プランレビューでの文面承認、(c) 日常運用での挙動観測。ヘッドレス A/B は実施しない(エンジニア判断 2026-07-11)。

**Per-task verification command**(コミット前に必須):
```sh
cd /Users/sakumatomoya/workspace/dotfiles && wc -l claude/CLAUDE.global.md
```
Expected: Task 1 完了後 `167 claude/CLAUDE.global.md`。

---

## 設計判断の出典

2026-07-11 の /design-discussion 2 ラウンド。第 1 ラウンド(Fable 前提再調整): モデル固定は現状維持(lead=Fable 5 / teammates=Opus 4.8、メモリ `agent-model-pinning-decision`)、蒸留規範は保持、boundaries/再接地/委譲解禁を追加。第 2 ラウンド(SSOT 再設計): code-architect 分析で乖離 7 項目(orchestrator 文、one retry、3/11 エージェント、teammate 像欠落、options 提示矛盾、モデルポリシー不在、死んだ dispatching-parallel-agents)を確認し、複写構造が乖離の発生源と判断して方針 B(SSOT 化+正確化)を採択。あわせて「解説直後の即 ask」問題の発生源(RUA §3 と design-discussion の multiple-choice 優先)を修正対象に追加。

根拠となる公式ドキュメント: [Prompting Claude Fable 5](https://platform.claude.com/docs/en/build-with-claude/prompt-engineering/prompting-claude-fable-5) — 「steer most behaviors with a brief instruction rather than enumerating each behavior by name」(Strong instruction following)、「Pause for the user only when the work genuinely requires them」(checkpoint)、「give a recommendation, not an exhaustive survey」(Longer turns)、「State the boundaries」、「Parallel subagents」、「Skills developed for prior models are often too prescriptive … can degrade output quality」(Recommended scaffolding changes)。

---

### Task 1: CLAUDE.global.md の SSOT 再設計(11 編集)

**Why:** 乖離 3 例(one retry / code-reviewer×review / 3/11 エージェント)は全て「スキル進化時に global の機構複写を直し忘れた」結果であり、正確化だけでは再発する。global を契約に限定して機構の複写を除去し、同時に公式 Fable ガイド準拠の規範(boundaries / 再接地 / 委譲 / ask 条件化)を統合する。

**Behavior change:** yes(lead の委譲・質問・報告挙動、triage/ループ記述の参照構造)
**Discipline:** 直接編集+構造チェック(共通注記参照)。

**Files:**
- Modify: `claude/CLAUDE.global.md`(11 箇所)

### Steps

- [ ] **Step 1: ブランチ作成**

```sh
git checkout -b claude-global-ssot-redesign
```

- [ ] **Step 2: 前文圧縮(−1 行)**

Before:
```markdown
This document defines how Claude Code should behave when interacting with the user.
If a project-level CLAUDE.md exists, its guidelines take precedence over this document for project-specific concerns.
```
After:
```markdown
This document defines how Claude Code should behave when interacting with the user. A project-level CLAUDE.md takes precedence for project-specific concerns.
```

- [ ] **Step 3: RUA §3 — ask 形式処方の除去(±0 行)**

Before:
```markdown
3. **Ask the one question that matters most.** If real uncertainty remains after investigating, ask exactly one question — the one whose answer most changes what you do next; multiple-choice with a recommendation and its trade-off preferred. If investigation resolved the ambiguity, skip the question and proceed (or route to `/design-discussion`).
```
After:
```markdown
3. **Ask only what the engineer must decide.** If real uncertainty remains after investigating, give a recommendation with its trade-off — not an exhaustive survey — and ask the question whose answer most changes what you do next. Leave room for discussion; the engineer decides when a decision is made. If investigation resolved the ambiguity, proceed (or route to `/design-discussion`).
```

- [ ] **Step 4: Response Quality — 再接地の統合(±0 行)**

Before:
```markdown
- **The final message stands alone.** Everything the engineer needs from a turn — findings, conclusions, decisions needed — must appear in that turn's last message.
```
After:
```markdown
- **The final message stands alone.** Everything the engineer needs from a turn — findings, conclusions, decisions needed — must appear in that turn's last message. After a long autonomous stretch, write it as a re-grounding for a reader who saw none of the work: outcome first, working shorthand and invented labels dropped.
```

- [ ] **Step 5: 自律ループ bullet の契約化(±0 行)**

Before:
```markdown
- Running an autonomous loop within `execute-plan`: per-task implementation → review via agent-teams, including one retry on failure
```
After:
```markdown
- Running the autonomous loop within `execute-plan` as the executing skill defines it — the engineer's contract is the Escalation Rule, not the loop's internals
```

- [ ] **Step 6: Boundaries 小節の追加(+5 行)**

Step 5 適用後の bullet と `### Escalation Rule` の間に挿入:

```markdown
### Boundaries

- When the engineer describes a problem, asks a question, or thinks out loud rather than requesting a change, the deliverable is the assessment. Report findings and stop; apply a fix only when asked.
- Before running a command that changes system state (restarts, deletes, config edits), check that the evidence supports that specific action — a signal that pattern-matches a known failure may have a different cause.
```

出典: 公式「State the boundaries」スニペットの圧縮版。

- [ ] **Step 7: orchestrator 文の書き直し(±0 行)**

Before:
```markdown
The engineer is the orchestrator of AI agents — a tech lead who decides which agents to deploy, when, and in what combination.
```
After:
```markdown
The engineer owns the loops; Claude Code operates them. The engineer sets direction, approves plans, and rules on escalations at the phase gates; between gates, work runs autonomously, and which agents run inside a phase — and how — is defined by the executing skill.
```

(2026-07-11 walkthrough 中のエンジニア決定: 「orchestrator of loops」案は lead が実時間の orchestration を担う実態と不一致、かつ「decides which phase runs next」は自動遷移の存在と矛盾するため、所有(エンジニア)と運転(Claude Code)を分離する案 1 に差し替え。)

- [ ] **Step 8: Triage のポインタ化(−5 行)**

Before(6 行、Core Flow 節内):
```markdown
**Triage classification** (applied by Claude Code to each review item):
- **Push back** — already decided (Design Doc, Design Discussion record, plan's "Alternative Solutions" / "Out of scope"), violates YAGNI, technically incorrect, or reviewer lacks context. Rejected within the loop; cite the decision source.
- **Fix** — minor improvements, bugs, or quality items within the existing design. Appended to the plan; flow returns to `execute-plan` autonomously.
- **Escalate** — requires architecture changes, Design Doc contract changes, scope expansion beyond the plan, or substantive new evidence overturning a prior decision. Reported to the engineer; loop stops.

Already-decided items are never escalated; minor fixes never trigger escalation. The loop continues until `review` reports no remaining items.
```
After(1 行):
```markdown
**Triage** (applied by Claude Code to each review item) resolves to **Push back** (rejected in-loop, citing the decision source), **Fix** (appended to the plan; `execute-plan` re-entry), or **Escalate** (reported to the engineer; loop stops). Classification criteria live in `receiving-code-review`; the contract here: already-decided items are never escalated, minor fixes never trigger escalation, and the loop continues until `review` reports no remaining items.
```

- [ ] **Step 9: Entry Point に design-doc 経路を追加(±0 行)**

Before:
```markdown
All work begins with `/design-discussion`. The discussion identifies the nature of the work and routes onward (`create-plan` for any implementation work, `systematic-debugging` for bugs). Every change — including trivial ones — flows through `/create-plan → /execute-plan` to preserve the autonomous loop discipline.
```
After:
```markdown
All work begins with `/design-discussion`. The discussion identifies the nature of the work and routes onward (`design-doc` → `create-plan` when the design warrants formal documentation, `create-plan` for other implementation work, `systematic-debugging` for bugs). Every change — including trivial ones — flows through `/create-plan → /execute-plan` to preserve the autonomous loop discipline.
```

- [ ] **Step 10: Cross-cutting Skills から dispatching-parallel-agents を除去(−1 行)**

削除する行:
```markdown
- `dispatching-parallel-agents` — invoked when multiple independent problems can be addressed in parallel
```

理由: どのスキルからも配線されていない(code-architect 分析 §3a)。スキルファイル自体の処遇は Out of scope。

- [ ] **Step 11: Rules 3 bullets の書き直し(±0 行)**

Before:
```markdown
- Do not launch agents or invoke skills speculatively. Only when the engineer requests it or when a skill's transition explicitly calls for it.
- When multiple skills or agents could be useful, present the options and let the engineer decide.
- Each agent operates in isolation. Pass necessary context explicitly — agents cannot read the current conversation.
```
After:
```markdown
- Skills and state-changing work are never invoked speculatively — only when the engineer requests it or a skill's transition calls for it. Read-only investigation (searches, code exploration, summarization) may be delegated to subagents freely and asynchronously — keep working while they run.
- At phase boundaries, when multiple skills could apply, present the options and let the engineer decide. Inside autonomous loops, triage decides per Core Flow.
- Agents and teammates never see this conversation — pass each the context it needs. How a skill coordinates its agents (one-shot subagents vs a persistent team) is defined by that skill.
```

出典: 公式「Parallel subagents」(bullet 1)、乖離 F の限定(bullet 2)、乖離 C の契約化(bullet 3)。

- [ ] **Step 12: Available Agents → Agents ポリシー段落(−2 行)**

Before(5 行):
```markdown
### Available Agents

- `code-architect` — Explores and analyzes codebase architecture. Called from `design-discussion` or `systematic-debugging` when structural context is needed.
- `implementation-verifier` — Verifies implementation quality. Called by the `/verify` skill.
- `code-reviewer` — Reviews code changes against specifications and quality standards. Called by `agent-teams-driven-development` and `review`.
```
After(3 行):
```markdown
### Agents

Agent definitions live in `agents/` and are owned by the skills that launch them — who launches what is defined there, not here. Model policy: all agents and teammates are pinned to opus; they do not inherit the session model.
```

- [ ] **Step 13: 構造チェック**

```sh
cd /Users/sakumatomoya/workspace/dotfiles
wc -l claude/CLAUDE.global.md
grep -c '^### Boundaries$' claude/CLAUDE.global.md
grep -c '^### Agents$' claude/CLAUDE.global.md
grep -c 'owns the loops' claude/CLAUDE.global.md
grep -c 'Read-only investigation' claude/CLAUDE.global.md
grep -c 're-grounding' claude/CLAUDE.global.md
! grep -q 'Available Agents' claude/CLAUDE.global.md && echo OK1
! grep -q 'one retry' claude/CLAUDE.global.md && echo OK2
! grep -q 'multiple-choice' claude/CLAUDE.global.md && echo OK3
! grep -q 'dispatching-parallel-agents' claude/CLAUDE.global.md && echo OK4
! grep -q 'orchestrator of AI agents' claude/CLAUDE.global.md && echo OK5
```

Expected: `167` / `1` / `1` / `1` / `1` / `1` / `OK1`〜`OK5`。

- [ ] **Step 14: Commit**

```sh
git add claude/CLAUDE.global.md claude/docs/plans/2026-07-11-claude-global-ssot-redesign.md
git commit -m "$(cat <<'EOF'
Update: CLAUDE.global.md を SSOT 再設計(契約=global、機構=スキル)

Fable 運用継続を前提に、公式 Prompting Claude Fable 5 の原則(brief
instruction > 列挙 / 停止条件を書き形式を書かない / 旧モデル向け足場
の撤去)に従い再設計する:
- orchestrator 文を「エンジニアが所有、Claude Code が運転」に書き直し
- 自律ループ bullet を契約化し「one retry」の機構複写を除去
- triage は 3 分類の決定と不変条件のみ残し、基準は
  receiving-code-review を単一情報源に
- Available Agents 列挙(3/11 で陳腐化)を廃止し、所有スキル参照+
  モデル固定ポリシーの 1 段落に置換
- Rules を書き直し: 読取専用調査の非同期委譲を解禁、options 提示は
  フェーズ境界に限定、teammate 協調はスキル定義を正とする
- RUA §3 の multiple-choice 形式処方を除去(解説直後の即 ask 誘発)
- boundaries 2 文・最終メッセージ再接地・前文圧縮を統合

行数 171 → 167(予算 ≤175)。乖離の発生源だった機構複写を構造的に
除去する。

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
EOF
)"
```

---

### Task 2: 周辺 3 ファイルの外科修正(stale 記述+ask 形式処方の発生源)

**Why:** code-reviewer / code-architect の frontmatter は global と同じ stale な起動元記述を持ち(乖離 A/G)、global 側だけ直すと矛盾が残る。design-discussion の multiple-choice 優先 2 記述は「解説直後の即 ask」問題の直接の発生源であり、スキル監査(繰延)を待たず外科的に塞ぐ(エンジニア承認済み)。

**Behavior change:** yes(design-discussion 中の質問カデンツ、エージェント選択時の参照情報)
**Discipline:** 直接編集+構造チェック(共通注記参照)。

**Files:**
- Modify: `claude/agents/code-reviewer.md`(frontmatter description 1 行)
- Modify: `claude/agents/code-architect.md`(frontmatter description 1 行)
- Modify: `claude/skills/design-discussion/SKILL.md`(2 箇所)

### Steps

- [ ] **Step 1: code-reviewer.md — /review 使用の誤記述を修正**

Before:
```markdown
  Used by agent-teams-driven-development (spec / code-quality reviewer roles) and the /review skill.
```
After:
```markdown
  Used by agent-teams-driven-development (spec / code-quality reviewer roles); /review uses its own dedicated reviewer agents instead.
```

- [ ] **Step 2: code-architect.md — 未配線の systematic-debugging 参照を除去**

Before:
```markdown
  Called from /design-discussion or /systematic-debugging when structural context is needed.
```
After:
```markdown
  Called from /design-discussion when structural context is needed.
```

- [ ] **Step 3: design-discussion SKILL.md — Operating Procedure step 3 の形式処方を置換**

Before(60〜61 行目、折返し込み):
```markdown
   time — the single highest-leverage question next; multiple-choice or
   recommend-an-answer formats preferred. Walk branch by branch,
```
After:
```markdown
   time — the single highest-leverage question next; recommend an
   answer with its trade-off and let discussion settle before moving
   on. Walk branch by branch,
```

- [ ] **Step 4: design-discussion SKILL.md — Key Principles の Multiple choice preferred を置換**

Before:
```markdown
- **Multiple choice preferred** — Easier to answer than open-ended.
```
After:
```markdown
- **Explain first, decide later** — Present analysis and leave room for discussion; offer options when the engineer is ready to decide.
```

- [ ] **Step 5: 構造チェック**

```sh
cd /Users/sakumatomoya/workspace/dotfiles
! grep -q 'and the /review skill' claude/agents/code-reviewer.md && echo OK1
! grep -q '/design-discussion or /systematic-debugging' claude/agents/code-architect.md && echo OK2
! grep -qi 'multiple.choice' claude/skills/design-discussion/SKILL.md && echo OK3
grep -c 'Explain first, decide later' claude/skills/design-discussion/SKILL.md
```

Expected: `OK1` / `OK2` / `OK3` / `1`。

- [ ] **Step 6: Commit**

```sh
git add claude/agents/code-reviewer.md claude/agents/code-architect.md claude/skills/design-discussion/SKILL.md
git commit -m "$(cat <<'EOF'
Fix: stale な起動元記述と ask 形式処方をスキル/エージェント側で修正

- code-reviewer: /review では未使用(専用フリートが担当)を明記
- code-architect: 未配線の /systematic-debugging 参照を除去
- design-discussion: multiple-choice 優先の 2 記述を「推奨+議論の
  余地」に置換。解説直後に選択 UI を出して議論を閉じる挙動の発生源
  だった(スキル監査本体は別イテレーション、これは外科修正)

Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>
EOF
)"
```

---

## Final verification (after all tasks)

```sh
cd /Users/sakumatomoya/workspace/dotfiles
wc -l claude/CLAUDE.global.md          # 167
bash claude/install.sh                 # 配布(~/.claude/ へ反映)
diff claude/CLAUDE.global.md ~/.claude/CLAUDE.md && echo SYNC-GLOBAL-OK
diff claude/skills/design-discussion/SKILL.md ~/.claude/skills/design-discussion/SKILL.md && echo SYNC-SKILL-OK
diff claude/agents/code-reviewer.md ~/.claude/agents/code-reviewer.md && echo SYNC-AGENT-OK
```

Expected: 行数 167、install.sh 正常終了、`SYNC-GLOBAL-OK` / `SYNC-SKILL-OK` / `SYNC-AGENT-OK`。

## Post-/review iteration

(直接編集方式のため /review は実行しない。エンジニアが PR diff を目視レビューし、指摘があればここに修正タスクを追記する。)

## Push and PR

```sh
git push -u origin claude-global-ssot-redesign
gh pr create --base main --title "CLAUDE.global.md: SSOT 再設計 — 契約=global/機構=スキル、Fable 規範統合" --body "..."
```

PR 説明は PRDoc 形式(概要/利用側への影響/設計判断/変更内容/テスト/スコープ外/参考資料)。「設計判断」節に code-architect 分析の乖離 7 項目と方針 B 採択理由、「テスト」節に A/B 不実施(エンジニア判断 2026-07-11)と構造チェック結果、「参考資料」に Prompting Claude Fable 5 / Introducing Claude Fable 5 をリンク。

## Out of scope

- **スキル 17 個・エージェント定義の全面監査**(公式の de-prescribe 推奨)— 別イテレーション。design-discussion の 2 行のみ本プランで前倒し
- **dispatching-parallel-agents スキルファイル自体の削除/再配線** — global リストからの除去のみ実施。ファイル処遇は監査時に判断
- **systematic-debugging への code-architect 配線** — 記述を実態に合わせるに留める。配線するかは監査時に判断
- **review/SKILL.md 側の triage 記述の統合** — triage 三重記述のうち global 分のみ解消。review スキル側の複写は監査時に receiving-code-review へ統合検討
- **モデル固定の変更**(Sonnet 5 切替 / 継承復帰)— 繰延。メモリ `agent-model-pinning-decision` 参照
- **teammate 間通信の英語統一**(CJK ツールコール不具合への耐性)— 監査時に検討
- **公式スニペットのうちハーネス搭載済み・N/A 分の移植**(memory / send_to_user / effort / context-anxiety)

## Alternative Solutions Considered

- **A: 正確化のみ(構造維持)**: 乖離 7 項目を現行構造のまま修正。**Rejected because**: 乖離 3 例は全て機構複写の直し忘れが原因で、複写構造を残すと再発する。11 エージェント列挙で行数超過(概算 185 行)。公式の「brief instruction > 列挙」にも反する。
- **C: Loop Engineering 全面再構成**: 文書構造と運用モデル自体の再設計。**Deferred because**: A/B 実証済みの文面を広範囲に揺らし Design Doc 規模になる。B 完了後に必要性を再評価。
- **design-discussion 修正のスキル監査までの繰延**: **Rejected because**: 「解説直後の即 ask」問題の直接の発生源であり、RUA §3 だけ直しても挙動が変わらない(エンジニア承認で外科的前倒し)。
- **旧プラン(Fable 差分 4 編集のみ)**: **Superseded** — 本プランに統合(boundaries / 再接地 / 前文圧縮は同一、委譲は Rules 書き直しに吸収)。
