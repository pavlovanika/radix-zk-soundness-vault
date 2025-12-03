// app_ux.nr
// Extended architecture gauge for zk-arch-noir-gauge
//
// Original repo models 3 axes:
//   - privacy   (0–100)
//   - FHE       (0–100)
//   - soundness (0–100)
// and selects between 3 styles:
//   0 = Aztec-style zk privacy rollup
//   1 = Zama-style FHE compute stack
//   2 = Soundness-first protocol
//
// This extended circuit adds a 4th axis:
//   - ux_speed (0–100) = UX, latency, "DeFi-feel"
//
// Styles (indices) stay the same (0,1,2) but the scoring
// now takes ux_speed into account as a soft tie-breaker / bonus.

fn main(
    privacy: pub u32,
    fhe: pub u32,
    soundness: pub u32,
    ux_speed: pub u32,
) -> pub u32 {
    // Compute style scores with the extended heuristic
    let (aztec_score, zama_score, soundness_score) =
        compute_extended_scores(privacy, fhe, soundness, ux_speed);

    // Select the best style index with the same tie-breaking priority:
    //   Aztec (0) > Zama (1) > Soundness (2)
    let style = select_style_with_priority(
        aztec_score,
        zama_score,
        soundness_score,
    );

    style
}

// ---------------------------
// Internal helpers
// ---------------------------

// Extended scoring that includes UX / speed.
//
// Heuristic:
//   - Aztec-style: strong privacy, decent soundness, UX bonus
//   - Zama-style: strong FHE, some privacy, little UX
//   - Soundness-first: strongest soundness, small UX bonus
//
// All inputs expected in [0, 100]. Weights chosen so scores
// stay comfortably in u32 range.
fn compute_extended_scores(
    privacy: u32,
    fhe: u32,
    soundness: u32,
    ux_speed: u32,
) -> (u32, u32, u32) {
    // Cast to a wider type if you want, but u32 is already fine
    // given the small ranges and weights here.

    // Original idea from README (no UX):
    //   Aztec     = 3 * privacy + 2 * soundness
    //   Zama      = 2 * privacy + 3 * FHE
    //   Soundness = 4 * soundness + 1 * privacy
    //
    // We just add a small UX term to each, with different weights.

    let aztec_score: u32 =
        3u32 * privacy +
        2u32 * soundness +
        1u32 * ux_speed;      // likes decent UX too

    let zama_score: u32 =
        2u32 * privacy +
        3u32 * fhe +
        0u32 * ux_speed;      // Zama-style = heavy compute focus

    let soundness_score: u32 =
        4u32 * soundness +
        1u32 * privacy +
        1u32 * ux_speed;      // high-assurance chains still care a bit about UX

    (aztec_score, zama_score, soundness_score)
}

// Tie-breaking selection logic:
//
// Keeps the original priority from the README:
//   - Aztec wins ties vs Zama and Soundness
//   - Zama wins ties vs Soundness
fn select_style_with_priority(
    aztec_score: u32,
    zama_score: u32,
    soundness_score: u32,
) -> u32 {
    // Start assuming Aztec is best
    let mut best_index: u32 = 0;
    let mut best_score: u32 = aztec_score;

    // Check Zama: replace only if strictly better (so Aztec wins ties)
    if zama_score > best_score {
        best_score = zama_score;
        best_index = 1;
    }

    // Check Soundness: replace only if strictly better than current best.
    // Since we check after Zama, this means:
    // - Zama wins ties vs Soundness
    // - Aztec still wins ties vs both.
    if soundness_score > best_score {
        best_score = soundness_score;
        best_index = 2;
    }

    best_index
}

// Optional helper: expose the raw scores as a pure function.
// This is NOT an entrypoint; you can import and call it from
// other Noir circuits if you want richer composition.
fn raw_scores(
    privacy: u32,
    fhe: u32,
    soundness: u32,
    ux_speed: u32,
) -> (u32, u32, u32) {
    compute_extended_scores(privacy, fhe, soundness, ux_speed)
}
