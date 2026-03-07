# SoroRail

**Payment rails for Stellar.** Audited Soroban contracts, a typed client SDK, and a reference application — so that teams building payroll, subscriptions, escrow, or streaming on Stellar stop rewriting the same contracts from scratch.

> **This document is a build specification.** It describes an entire GitHub organization across five repositories. It is written to be handed to Claude Code repo by repo. Do not attempt to build all five at once. Start with `contracts`, finish it, then move down the list. Each repo section below is self-contained enough to act as that repo's own README once the org is live.

---

## Table of contents

- [Why this exists](#why-this-exists)
- [Organization structure](#organization-structure)
- [Dependency graph](#dependency-graph)
- [Build order and prompting strategy](#build-order-and-prompting-strategy)
- [Repo 1 — `contracts`](#repo-1--contracts)
- [Repo 2 — `sdk`](#repo-2--sdk)
- [Repo 3 — `app`](#repo-3--app)
- [Repo 4 — `docs`](#repo-4--docs)
- [Repo 5 — `.github`](#repo-5--github)
- [Conventions that apply to every repo](#conventions-that-apply-to-every-repo)
- [Security posture](#security-posture)
- [Roadmap](#roadmap)
- [Verify before you build](#verify-before-you-build)

---

## Why this exists

Across the Stellar ecosystem the same payment logic is being reimplemented in isolation. Payroll systems, streaming protocols, group-settlement engines, and escrow marketplaces each hand-roll their own vesting math, their own authorization checks, their own batch disbursement loops. Each implementation carries its own bugs, and none of them are audited.

SoroRail is the shared layer underneath all of that: a small set of composable, well-tested Soroban contracts covering the payment patterns that recur in nearly every real application, plus a typed TypeScript client so frontend developers never touch XDR by hand.

The design target is `openzeppelin-contracts` for Stellar payments — boring, correct, obvious to audit, and easy to depend on.

**On the name.** A rail is the boring, load-bearing thing underneath a payment system that nobody thinks about until it breaks. That is the ambition. The `soro-` prefix marks it as Soroban-native; note that SDF increasingly brands the platform "Stellar smart contracts" in product surfaces while retaining "Soroban" in developer documentation, so prefer "Stellar smart contracts" in user-facing copy and "Soroban" in developer-facing copy.

**Non-goals.** This is not a wallet. It is not an anchor or an on/off-ramp. It does not do KYC, custody, fiat rails, or DEX routing. It does not issue tokens. Scope discipline is the point; proposals that expand this list should be declined.

---

## Organization structure

GitHub organization: **`sororail`**

| Repo | Language | Purpose | Depends on |
|---|---|---|---|
| `contracts` | Rust / Soroban | The payment contracts. The core deliverable. | — |
| `sdk` | TypeScript | Typed client for the contracts. Framework-agnostic. | `contracts` |
| `app` | TypeScript / Next.js | Reference fullstack application. | `sdk` |
| `docs` | MDX / Astro Starlight | Documentation site. | `contracts`, `sdk` |
| `.github` | Markdown / YAML | Org profile, shared templates, shared workflows. | — |

Each repo is separately claimable on Drips and separately listable on GrantFox. Each carries its own `FUNDING.json`, its own issue tracker, and its own release cadence.

### Why separate repos rather than a monorepo

- Drips treats a GitHub repository as the fundable unit. Five repos means five projects in the dependency tree, and `sdk` can list `contracts` as a dependency so a share of anything the SDK earns flows upstream automatically.
- Contributors self-select by stack. A Rust engineer subscribes to `contracts` and is never notified about a CSS change.
- Release cadences genuinely differ. Contracts should move slowly and deliberately; the app can move daily.
- The blast radius of a bad merge is contained.

The cost is cross-repo coordination, handled by publishing the SDK to npm and the contract WASM to GitHub Releases rather than by path references.

---

## Dependency graph

```
                    contracts  (Rust, Soroban)
                        │
                        │  publishes: WASM + contract specs
                        │             to GitHub Releases
                        ▼
                      sdk  (TypeScript)
                        │
                        │  publishes: @sororail/sdk to npm
                        │
            ┌───────────┴───────────┐
            ▼                       ▼
          app                     docs
     (Next.js reference)     (Starlight site)
```

Each downstream repo declares its upstream in `FUNDING.json` splits. Suggested split for `sdk`: 70% maintainers, 30% to `contracts`. For `app`: 60% maintainers, 40% to `sdk`.

---

## Build order and prompting strategy

Build strictly in this order. Do not begin a repo until the one above it has a green CI run and a tagged release.

1. **`.github`** — half a day. Establishes templates and conventions everything else inherits.
2. **`contracts`** — the bulk of the work. Nothing downstream can be correct until these are.
3. **`sdk`** — generated in part from the contract specs produced in step 2.
4. **`app`** — consumes the published SDK.
5. **`docs`** — written last, when the API has stopped moving.

**When prompting Claude Code:** open a separate session per repo, in that repo's own directory. Paste only that repo's section from this document plus the [Conventions](#conventions-that-apply-to-every-repo) and [Verify before you build](#verify-before-you-build) sections. Feeding the whole document produces shallow work across five repos instead of one finished repo.

Within `contracts`, prompt one contract at a time — `token_utils`, then `escrow`, then `stream`, then `vesting`, then `recurring`, then `batch_payout`. Each must be complete with tests before starting the next.

---

## Repo 1 — `contracts`

> Composable Soroban payment contracts. Rust, `no_std`, audited-in-intent.

### Stack

- Rust, edition 2021
- `soroban-sdk` (see [Verify before you build](#verify-before-you-build) for version)
- `stellar` CLI for build, deploy, and invoke
- `cargo-nextest` for the test runner
- Workspace layout: one crate per contract, plus one shared crate
- Crates published to crates.io as `sororail-common`, `sororail-escrow`, `sororail-stream`, `sororail-vesting`, `sororail-recurring`, `sororail-batch-payout`. Reserve all six names early.

### Layout

```
contracts/
├── Cargo.toml                 # workspace root
├── rust-toolchain.toml        # pin the toolchain
├── Makefile                   # build / test / fmt / lint / optimize
├── FUNDING.json
├── contracts/
│   ├── common/                # shared types, errors, events, guards
│   ├── escrow/
│   ├── stream/
│   ├── vesting/
│   ├── recurring/
│   └── batch_payout/
├── tests/                     # cross-contract integration tests
└── .github/workflows/ci.yml
```

Every contract crate follows the same internal shape: `lib.rs`, `contract.rs`, `storage.rs`, `types.rs`, `errors.rs`, `events.rs`, `test.rs`.

### The contracts

Build in the order given. Each is independently deployable; composition happens at the call site, not through inheritance.

#### `common`

Not a contract — a shared library crate.

- `Error` enum with a stable, documented numeric mapping. Never renumber a released variant.
- Storage key enums and TTL/bump helpers. Soroban state expires; every contract must extend TTL on access, and this logic lives here so it is written once.
- `require_auth` guard helpers.
- Event emission helpers with a consistent topic scheme.
- Basis-point math with explicit overflow handling. No silent saturation.

#### `escrow`

Funds held by the contract, released on a condition.

- `init(depositor, beneficiary, arbiter: Option<Address>, token, amount, deadline)`
- `fund()` — pulls tokens from depositor
- `release()` — beneficiary receives; callable by depositor or arbiter
- `refund()` — callable by depositor after deadline, or by arbiter
- `dispute()` / `resolve(split_bps)` — arbiter splits between parties
- States: `Created → Funded → (Released | Refunded | Disputed → Resolved)`. Illegal transitions must error, not panic.

#### `stream`

Continuous per-second transfer from sender to recipient.

- `create(sender, recipient, token, rate_per_second, start, stop, cancellable: bool)`
- `withdraw(amount: Option<i128>)` — recipient claims accrued balance; `None` claims all available
- `cancel()` — settles accrued amount to recipient, returns remainder to sender; only if `cancellable`
- `balance_of(who)` — view; accrued but unwithdrawn
- `top_up(amount)` / `extend(new_stop)`
- Accrual is computed from ledger timestamp at read time. Never write per-second state.
- The critical correctness property: withdrawn + refunded + remaining always equals deposited, exactly, with no rounding leakage. Test this as an invariant.

#### `vesting`

Scheduled release against a schedule, with a cliff.

- `create(grantor, beneficiary, token, total, start, cliff, duration, revocable: bool)`
- `claim()` — beneficiary withdraws vested-but-unclaimed
- `revoke()` — grantor reclaims unvested; vested portion stays claimable
- `vested_amount(at: u64)` — pure view, testable in isolation
- Linear vesting after cliff. Nothing claimable before cliff. Fully vested at `start + duration`.

#### `recurring`

Pull-based authorization for subscriptions. The payer authorizes a cap and cadence; the payee pulls within it.

- `authorize(payer, payee, token, amount_per_period, period_seconds, max_periods: Option<u32>)`
- `charge()` — callable by payee, at most once per elapsed period
- `cancel()` — callable by either party, effective immediately
- `next_chargeable_at()` — view
- Must not accrue chargeable periods retroactively when a payee skips one. A payee who forgets to charge for three months cannot then charge three times. This is a deliberate consumer-protection choice; document it prominently.

#### `batch_payout`

One transaction, many recipients. The payroll primitive.

- `execute(funder, token, recipients: Vec<(Address, i128)>)`
- `execute_equal(funder, token, recipients: Vec<Address>, amount_each: i128)`
- Enforce a documented maximum recipient count determined by resource limits, discovered empirically and asserted in a test — not guessed.
- All-or-nothing. A single failed transfer reverts the batch.

### Testing requirements

Non-negotiable, and the main thing that makes this credible as a dependency:

- Unit tests per contract using `soroban_sdk::testutils`, with time advanced via the test ledger rather than mocked clocks.
- Authorization tests: every privileged entry point must have a test asserting it fails for an unauthorized caller. Missing auth checks are the most common Soroban vulnerability class.
- Arithmetic edge cases: zero amounts, `i128::MAX`, one-second durations, cliff equal to duration, stop before start.
- Conservation invariants for `stream` and `vesting`, as described above.
- Integration tests in `/tests` that deploy real token contracts and exercise full lifecycles.
- Target ≥90% line coverage, enforced in CI.

### Deliverables

- All six crates building to optimized WASM
- `make test` green
- Published contract specs as JSON, attached to a GitHub Release
- Deployed to Stellar **testnet**, with addresses recorded in `DEPLOYMENTS.md`
- No mainnet deployment until an external audit is complete. State this in the README.

---

## Repo 2 — `sdk`

> `@sororail/sdk` — typed TypeScript client. No React, no framework assumptions.

### Stack

- TypeScript, strict mode
- `@stellar/stellar-sdk`
- `tsup` for bundling, dual ESM/CJS output
- `vitest` for tests
- `changesets` for versioning and release
- Published as `@sororail/sdk`. Reserve the `@sororail` npm scope before the first release.

### Design rules

- Zero framework dependencies. React helpers, if ever built, go in a separate `@sororail/react` package in this same repo.
- Every contract gets a client class: `EscrowClient`, `StreamClient`, `VestingClient`, `RecurringClient`, `BatchPayoutClient`.
- Uniform method shape: build → simulate → sign → send → confirm, with each stage independently accessible. Callers who want to inspect a simulation before signing must be able to.
- Signing is injected, never performed by the SDK. Accept a `Signer` interface; ship adapters for Freighter and for a keypair signer used in tests.
- Contract errors are decoded into typed error classes with readable messages. A user must never see a bare error code.
- Amounts cross the API boundary as `bigint`, never `number`. Provide `toStroops` / `fromStroops` helpers and document the decimals trap loudly.

### Layout

```
sdk/
├── package.json
├── src/
│   ├── index.ts
│   ├── clients/          # one per contract
│   ├── signers/          # freighter, keypair
│   ├── errors/           # decoding + typed classes
│   ├── types/
│   └── utils/            # amounts, time, addresses
├── test/
└── examples/             # runnable node scripts, one per contract
```

### Testing

- Unit tests with mocked RPC for building and error decoding
- Integration tests against contracts deployed on testnet, in a separate CI job that is allowed to be slow
- Every public method needs a runnable example in `examples/`. These double as documentation and as smoke tests.

---

## Repo 3 — `app`

> Reference fullstack application. Proves the SDK works and shows teams how to use it.

This is the fullstack piece. It is a real application, not a demo page — but its purpose is demonstrative, so favour clarity of implementation over feature breadth.

### Stack

- Next.js, App Router, TypeScript
- `@sororail/sdk` from npm
- Freighter for wallet connection
- Postgres via Drizzle ORM — for indexed history and metadata only, never as a source of truth for balances
- A background indexer polling Soroban RPC for contract events
- Tailwind, with the design direction below

### Critical architectural rule

**The chain is the source of truth. The database is a cache.** Balances, stream states, and vesting positions are always read from the contract for display of current state. The database exists to make history queryable and to store off-chain metadata such as recipient names, invoice notes, and email addresses. Any screen showing money must be reconcilable against chain state, and there should be a visible way to trigger that reconciliation.

### Features

**Wallet and account**
- Connect via Freighter, network detection with a hard warning on network mismatch
- Testnet faucet link and clear testnet-only banner

**Payroll** (`batch_payout` + `recurring`)
- Recipient list with CSV import
- Preview of total, per-recipient amounts, and estimated fees before signing
- Execute a batch payout; show per-recipient confirmation
- Schedule a recurring payroll run

**Streams** (`stream`)
- Create a stream with a live preview of the accrual curve
- Dashboard of incoming and outgoing streams, with balances ticking in real time
- Withdraw, top up, extend, cancel

**Vesting** (`vesting`)
- Create a grant with a visual schedule showing cliff and linear ramp
- Beneficiary view with claimable amount and next unlock
- Revoke, for revocable grants

**Escrow** (`escrow`)
- Create, fund, release, refund
- Optional arbiter with a dispute and split-resolution flow

**History**
- Unified event feed across all contracts, from the indexer
- CSV export

### Backend

- Route handlers under `app/api/`
- Indexer as a separate long-running process, not a serverless function. It must be resumable from the last processed ledger and idempotent on replay.
- Schema: `accounts`, `contacts`, `payment_runs`, `payment_run_items`, `indexed_events`, `sync_state`
- No private keys server-side, ever. All signing is client-side through the wallet. Say this explicitly in the README so no contributor is tempted.

### Design direction

The audience is finance and operations staff at small companies, plus the developers evaluating the SDK. The job is to make irreversible money movements feel inspectable before they happen.

- Confirmation before consequence: every state-changing action shows exactly what will happen — amounts, recipients, fees, and what cannot be undone — before the signing prompt.
- Numbers are the interface. Set monetary figures in a face with true tabular figures so columns align, and give amounts more typographic weight than the labels around them.
- Time is a first-class dimension for streams and vesting. Show schedules as actual schedules, not as progress bars.
- Empty states state the next action.
- Errors say what happened and what to do. Never surface a raw contract error code.
- Do not reach for the default AI-design palette. Pick a palette grounded in the subject — this is a ledger tool, and legibility under scrutiny matters more than personality. Spend boldness in one place only.

### Testing

- Playwright end-to-end tests against testnet contracts, covering at minimum: connect wallet, create stream, withdraw, create batch payout
- Component tests for amount input and formatting, which is where money bugs hide
- Seed script for local development

---

## Repo 4 — `docs`

> Documentation site.

- Astro Starlight, deployed to Cloudflare Pages or Vercel
- Sections: Getting started, Concepts (one page per payment primitive, explaining the pattern before the API), Contract reference, SDK reference, Guides, Security, Contributing
- Concepts pages must be readable by someone who has never used Soroban. This is the on-ramp.
- Every code sample must be extracted from a compiling example in `sdk/examples/`, not hand-written into the docs where it will rot.
- Include a page on the decimals trap and one on Soroban TTL/state expiry, since both bite every new integrator.

---

## Repo 5 — `.github`

> Org profile and shared configuration.

- `profile/README.md` — the org landing page. What sororail is, the five repos, how to start contributing, link to good-first-issues across the org.
- Shared issue templates: bug, feature, contract-proposal, docs
- PR template with a checklist: tests added, docs updated, no unrelated changes, breaking-change note
- `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`
- Reusable CI workflows referenced by the other repos
- Label taxonomy applied consistently org-wide: `good-first-issue`, `help-wanted`, `contract`, `sdk`, `app`, `docs`, `security`, `breaking`, and difficulty labels `size/s`, `size/m`, `size/l`

---

## Conventions that apply to every repo

**Licensing.** Apache-2.0 across the org. Add the license file before the first external contribution.

**Commits.** Conventional Commits. Enforced in CI.

**Branching.** Trunk-based. Short-lived branches, PRs into `main`, squash merge, linear history.

**CI must run on every PR.** Format check, lint, tests, build. Red CI blocks merge, with no exceptions for maintainers.

**Every repo gets, at minimum:** `README.md`, `LICENSE`, `CONTRIBUTING.md`, `SECURITY.md`, `FUNDING.json`, `.editorconfig`, a CI workflow, and issue/PR templates inherited from `.github`.

**`FUNDING.json`** goes on the default branch of every repo from the first commit, so each repo is claimable on Drips and can accrue before anyone has heard of it. Configure splits per the [dependency graph](#dependency-graph).

**Issue hygiene.** Every issue intended for outside contributors must state: the problem, the affected file paths, the expected behavior, acceptance criteria, and how to test the change locally. An issue that does not meet that bar is not ready to be labelled `good-first-issue`.

**Documentation is part of the definition of done.** A PR that changes public behavior without touching docs is incomplete.

---

## Security posture

State this plainly and repeatedly, including in the app UI:

- **Unaudited. Testnet only. Do not deploy to mainnet or handle real value until an external audit is complete.**
- No mainnet addresses published until then.
- `SECURITY.md` with a private disclosure path. Do not accept vulnerability reports through public issues.
- Run `cargo audit` in CI.
- Consider running CoinFabrik's Scout, which is an open-source static analyzer built for Soroban, as a CI step.
- When ready, the Stellar bug bounty covers Soroban platform exploits — worth reading the terms for how they scope contract-level findings.

The credibility of this project rests on not overstating its maturity. A library that says clearly what it has not yet proven is more trustworthy than one that stays quiet.

---

## Roadmap

**v0.1 — foundations.** `common`, `escrow`, `stream` with full tests. SDK clients for both. Testnet deployment.

**v0.2 — the payroll set.** `vesting`, `recurring`, `batch_payout`. SDK coverage complete. App: payroll and streams.

**v0.3 — reference app complete.** Indexer, history, all five primitives in the UI. Docs site live.

**v0.4 — hardening.** Fuzz testing, gas/resource benchmarking, external audit engagement, mainnet readiness review.

Deliberately excluded from the roadmap: multi-chain support, a token issuance module, fiat integration, a hosted service. If these come up in issues, decline them and say why.

---

## Verify before you build

This spec was written from a snapshot and the Soroban toolchain moves quickly. Before writing code, confirm the following against current sources rather than trusting anything here:

1. **Current `soroban-sdk` version** and whether the pinned Rust toolchain matches its requirements.
2. **Current `stellar` CLI syntax.** The CLI was renamed from `soroban` and command shapes have changed across releases; older tutorials will be wrong.
3. **Protocol version on testnet** and which host functions are available. Protocol 25 introduced cryptographic primitives that may be relevant to later work.
4. **Current `@stellar/stellar-sdk` major version** and its RPC client API surface.
5. **Resource limits** — instruction count, ledger entry size, transaction size. These determine the real maximum recipient count for `batch_payout`. Discover this empirically with a test that ramps recipient count until failure; do not hardcode a guess.
6. **Whether the Stellar Asset Contract interface has changed** for the token calls the contracts make.

Where current documentation contradicts this spec, current documentation wins. Note the discrepancy in the PR description so the spec can be corrected.

---

## Contributing

Issues labelled `good-first-issue` are scoped so that someone new to Soroban can complete them. Start there, comment to claim, and open a draft PR early — an in-progress PR with questions is more useful than a perfect one that arrives three weeks late.

If you are unsure whether something is in scope, open an issue before writing code.
