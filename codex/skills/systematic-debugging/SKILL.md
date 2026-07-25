---
name: systematic-debugging
description: Determine the root cause of a bug, test failure, or unexpected behavior before proposing or implementing a fix. Use whenever behavior is unexplained or a verification command fails unexpectedly.
---

# Systematic debugging

Do not patch symptoms before establishing a causal explanation.

## 1. Reproduce

- Capture the exact command, input, environment, expected result, and actual result.
- Confirm the failure is current and repeatable.
- Reduce the reproduction while preserving the failure.

If it cannot be reproduced, collect observations and state the uncertainty instead of guessing.

## 2. Trace the boundary

Follow data and control flow through each component boundary. Inspect logs, return values, persisted state, and side effects. Add temporary diagnostics only when they answer a specific question and remove them before completion.

## 3. Compare

Find a nearby working path, earlier revision, or minimal example. Enumerate meaningful differences and test one hypothesis at a time.

## 4. State the root cause

A root-cause statement must identify:

- the triggering condition;
- the incorrect assumption or implementation;
- the path from trigger to observed symptom;
- evidence that rules out leading alternatives.

## 5. Fix under authority

For a diagnosis-only request, report and stop. For an authorized fix:

1. write a regression test that demonstrates the root cause;
2. observe the red result;
3. implement the smallest causal fix;
4. run focused and relevant broader verification;
5. remove diagnostic artifacts.

If two materially different fixes fail, stop and report the evidence rather than stacking more guesses.
