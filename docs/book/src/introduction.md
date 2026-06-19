# Introduction

## Code has types — file systems do not

When writing Rust or TypeScript, the compiler tells you immediately when types don't match. Pass an `i32` where a `String` is expected and it is caught in an instant. Conventions encoded as types never stay implicit.

What about directory structure, though?

```text
src/
  components/
    Button/
      index.tsx
      Button.test.tsx      ← correct — component and test are paired
    Card/
      index.tsx            ← no test. Forgotten?
    Modal/
      index.tsx
      Modal.spec.tsx       ← wrong name (.test.tsx expected, .spec.tsx found)
  hooks/                   ← convention says this should live inside each page dir, not at the top
    graphql/
    user/
      useUser.ts
  pages/
    home.tsx
    Dashboard/
      index.tsx
    settings_page.tsx
  utils.ts
  helpers.ts               ← does this overlap with utils.ts?
```

The correspondence between source files and tests tends to drift. The test for `auth.ts` might be `auth.spec.ts` in one place and `auth.test.ts` in another. The `src/` hierarchy was supposed to be mirrored under `tests/`, but over time the structures diverge. These "intended shapes" erode silently because no one ever wrote them down.

With AI coding agents in the picture, the problem becomes even more visible. Even when you instruct the agent in a prompt, an LLM may create files and directories wherever it pleases. Catching every deviation through human review is not realistic. If the generated structure can be checked mechanically against a declared "type," any deviation by the agent is detected on the spot.

Conventions like "every component directory must contain a test file" or "`hooks/` must live inside each page's directory" — where are they documented? Usually in a corner of CONTRIBUTING.md, an old Slack thread, or someone's head.

Every new team member has to be told verbally, reviewers catch violations one by one, and the structure still drifts. It happens because directory structure has no type.

---

## Aside: a directory tree might be a forest, not a tree

We tend to picture a directory structure as a single tree branching from a root. But the moment you assign types to multiple trees and bundle them together, the result is no longer one tree — it is a list of trees, a *forest*.

The foundational concept behind dir-type is that forest. Rather than validating a single tree, dir-type treats a *collection* of trees as the typed unit.

---

## What is dir-type?

In one sentence: a tool for linting directory structure.

`dir-type` is a CLI tool written in Rust. It reads a YAML rule file and validates an actual directory tree against it, reporting any discrepancy as a diagnostic message. It never looks inside files — it focuses exclusively on **structure**.

By analogy with a type system, `.dir-type.yaml` is the "type definition file" for your directory structure. `dir-type check` runs the type check and reports errors with error codes and line numbers when something does not match.

The essential value of dir-type is that it lets you turn directory-structure conventions — previously buried in a corner of CONTRIBUTING.md, an old Slack thread, or someone's memory — into **an executable specification written in `.dir-type.yaml`**. Tacit knowledge becomes a versioned type definition that anyone can read and verify.
