# IcyMap / Procedural Microbial Ecosystem — Implementation Roadmap

## 1. Project Vision

Build a small, fully visible procedural ecosystem in which life begins with **one bacterium-like organism**.

The world is generated deterministically from a seed. Organisms live inside that world, acquire energy, move, reproduce by binary fission, inherit genes, mutate, compete for resources, die, and gradually form distinct lineages.

The organism model will use **real bacterial gene names where appropriate**, but each gene will initially be represented by a simplified normalized simulation value rather than literal DNA sequences or laboratory-scale gene-expression data.

Example:

```text
ftsZ activity = 0.52
```

means:

> In this simulation, this organism has a particular level of FtsZ-related division activity.

It does **not** mean that `0.52` is a real biological measurement.

The long-term goal is:

```text
Procedural world
      ↓
one bacterium
      ↓
energy + survival
      ↓
binary fission
      ↓
inheritance
      ↓
mutation
      ↓
selection
      ↓
different lineages / bacterial profiles
      ↓
ecosystem-level behaviour
```

---

# 2. Current State

## World generation

- [x] Seeded world generation
- [x] fBm height field
- [x] Moisture field
- [x] Terrain-detail noise
- [x] Island shaping
- [x] Deep water
- [x] Shallow water
- [x] Sand
- [x] Grass
- [x] Soil
- [x] Rock
- [x] Procedural flora
- [x] Irregular rock boundaries
- [x] Irregular soil/grass boundaries
- [x] Coast detection using neighboring shallow-water tiles
- [x] Terrain color variation using height/moisture

## Rendering / camera / UI

- [x] Camera movement
- [x] Camera clamping
- [x] World rendering
- [x] Flora rendering
- [x] HUD
- [x] FPS display
- [x] Mouse-to-world conversion
- [x] Tile inspector
- [x] Inspect height
- [x] Inspect moisture
- [x] Inspect biome
- [x] Inspect terrain
- [x] Inspect flora

## Terrain status

**Freeze terrain work here.**

Do not keep adding terrain features unless the organism simulation later exposes a concrete need.

---

# 3. Core Design Rule

The project should be built from **simple interacting systems**, not one giant organism AI.

An organism should not contain rules such as:

```text
if species == X:
    do X behaviour
```

Instead:

```text
Genome
   ↓
Phenotype
   ↓
Behaviour
   ↓
Environment
   ↓
Survival / reproduction
```

Different organisms should gradually become different because their inherited parameters differ.

---

# 4. Biological Abstraction

The simulation is biology-inspired, not molecularly exact.

## Gene representation

Initially:

```rust
Gene {
    activity: f32 // 0.0 .. 1.0
}
```

Example genome:

```text
Genome
└── ftsZ
      └── activity: 0.52
```

Later:

```text
Genome
├── division-related genes
├── motility-related genes
├── chemotaxis-related genes
├── stress-response genes
├── metabolism-related genes
└── mutation / replication-fidelity traits
```

Real bacterial genes can be introduced one at a time after their simulation role is defined.

## Important distinction

A gene should influence a **phenotypic rule**, rather than directly becoming behaviour.

Example:

```text
ftsZ activity
      ↓
division phenotype
      ↓
minimum division time / division efficiency / cost
      ↓
reproduction outcome
```

This gives room for trade-offs.

A value of `1.0` should not automatically mean "best".

Example:

```text
high division activity
+ potentially divides sooner
- may require more energy

low division activity
+ cheaper
- slower reproduction
```

Selection can then emerge from the environment.

---

# 5. Core Organism Model

Start extremely small.

```text
Organism
├── id
├── position
├── age
├── energy
├── alive
└── genome
```

Do **not** begin with:

- neural networks
- complicated memory
- sexes
- mating
- predators
- diseases
- hundreds of genes
- literal DNA sequences
- complex pathfinding

Those belong later, if needed.

---

# 6. Recommended Source Structure

Current files remain:

```text
src/
├── main.rs
├── camera.rs
├── generation.rs
├── render.rs
├── ui.rs
└── world.rs
```

Add systems only when their milestone begins:

```text
src/
├── organism.rs      # one organism / cell
├── genome.rs        # genes, inheritance, mutation
├── simulation.rs    # population update loop
├── resources.rs     # nutrient model, later
└── stats.rs         # population / lineage statistics, later
```

Do not create all files immediately.

---

# 7. Simulation Update Order

Eventually one simulation tick should conceptually be:

```text
WORLD
  ↓
resource state
  ↓
sense local environment
  ↓
organism movement
  ↓
resource intake
  ↓
energy/metabolism
  ↓
age/stress
  ↓
division check
  ↓
offspring creation
  ↓
mutation
  ↓
death
  ↓
population statistics
```

The exact implementation can evolve, but keeping a predictable update order will prevent simulation bugs.

---

# 8. Milestone 1 — Spawn One Organism

## Goal

Put exactly **one visible organism** into IcyMap.

## Implement

Create:

```text
organism.rs
```

Initial data:

```text
Organism
├── position
├── age
├── energy
└── genome
```

Add the first genome:

```text
Genome
└── ftsZ
```

Spawn one organism on a valid tile.

Render it as a simple circle.

## Do not add behaviour yet.

### Done when

- [ ] Program starts with one organism
- [ ] Organism appears at a deterministic valid position
- [ ] Organism renders separately from terrain/flora
- [ ] Tile inspector/world rendering still works
- [ ] Organism contains a genome
- [ ] Genome contains `ftsZ`

---

# 9. Milestone 2 — Age, Energy and Metabolism

## Goal

Make the organism a living simulation object rather than a static circle.

Each tick:

```text
age += dt
energy -= metabolism × dt
```

For the first test, metabolism can be a fixed constant.

Add death:

```text
if energy <= 0:
    organism dies
```

### Done when

- [ ] Age increases with simulation time
- [ ] Energy decreases
- [ ] Organism dies at zero energy
- [ ] Dead organisms are removed safely
- [ ] Population count can reach zero

---

# 10. Milestone 3 — Binary Fission

## Goal

Make the initial bacterium reproduce.

`ftsZ` becomes the first gene that affects phenotype.

Example conceptual mapping:

```text
ftsZ activity
      ↓
division readiness
```

Division should require at least:

```text
minimum age
+
minimum energy
+
ftsZ-derived division condition
```

When reproduction happens:

```text
Parent
energy = 100

        division

Parent               Child
energy = 60          energy = 40
```

The child initially receives the same genome.

Spawn the child beside the parent.

### Done when

- [ ] Start with exactly 1 organism
- [ ] Parent can divide
- [ ] Population becomes 2
- [ ] Both organisms continue updating
- [ ] They can divide again
- [ ] Energy is conserved according to the chosen division cost
- [ ] Offspring inherits `ftsZ`

---

# 11. Milestone 4 — Temporary Reproduction Test Loop

Before resources exist, temporarily allow organisms to gain energy at a controlled rate.

This is **only a wiring test**.

Example:

```text
energy += temporary_growth_rate × dt
```

This allows testing:

```text
1
↓
2
↓
4
↓
8
```

without implementing nutrients first.

## Important

Remove this artificial energy source as soon as reproduction is proven.

### Done when

- [ ] Population growth can be observed
- [ ] No borrow/index bugs appear while organisms reproduce
- [ ] New organisms are updated normally
- [ ] Population growth is deterministic for a fixed seed

---

# 12. Milestone 5 — Resource / Nutrient System

This replaces temporary passive energy.

The original ecosystem idea used food, water, hunger and thirst.

For the bacterial version, simplify these into:

```text
nutrient availability
+
environmental conditions
+
cell energy
```

Do not make bacteria literally eat trees.

The current terrain/flora system can initially influence **resource richness**.

Example:

```text
terrain
+
moisture
+
flora/environment
      ↓
local nutrient availability
```

Later this can become its own procedural nutrient field.

## First version

Each tile can expose a resource value.

Example:

```text
TileResource
├── nutrients
└── regeneration_rate
```

Organisms consume nutrients from the tile they occupy.

```text
nutrients decrease
organism energy increases
```

### Done when

- [ ] Passive energy gain is removed
- [ ] Organisms need environmental nutrients
- [ ] Consuming nutrients increases energy
- [ ] Nutrient supply can decrease
- [ ] Nutrients regenerate slowly
- [ ] Reproduction stops when resources are insufficient

This creates the first real carrying capacity.

---

# 13. Milestone 6 — Movement

## Goal

Allow organisms to move through the environment.

Start with simple stochastic movement.

```text
current position
      ↓
small random direction
      ↓
new position
```

Movement costs energy.

```text
distance moved
      ↓
movement energy cost
```

This immediately creates a trade-off:

```text
move more
+ find resources
- spend more energy
```

### Done when

- [ ] Organisms move
- [ ] Movement is frame-rate independent
- [ ] Movement costs energy
- [ ] Organisms stay inside the world
- [ ] Invalid terrain rules are respected

---

# 14. Milestone 7 — Sensing and Chemotaxis-Like Behaviour

This replaces the old generic "vision" concept with something more appropriate for bacteria.

An organism samples its nearby environment.

Example:

```text
left nutrient concentration
right nutrient concentration
forward nutrient concentration
```

Then movement can become weighted rather than random.

```text
higher nutrient concentration
        ↓
higher probability of moving that way
```

This is the bacterial equivalent of the earlier ecosystem's:

```text
vision
+
needs
+
weighted decisions
```

The organism still does not need a neural network.

### Done when

- [ ] Organism can sample nearby tiles/resources
- [ ] Movement responds to resource gradients
- [ ] Randomness is still present
- [ ] Better sensing can improve resource discovery

---

# 15. Milestone 8 — Genome Expansion

Once the complete loop works with `ftsZ`, add genes **one at a time**.

Do not add a large genome in one commit.

Target categories:

```text
Genome
├── division
├── movement
├── sensing
├── metabolism
├── stress response
└── replication / mutation behaviour
```

Each gene must answer:

1. What biological function is it representing?
2. What simulation phenotype does it change?
3. What benefit can it provide?
4. What cost or trade-off prevents "maximum = always best"?
5. What range does its simulation value use?

Example pattern:

```text
Gene activity
    ↓
phenotype calculation
    ↓
simulation behaviour
```

---

# 16. Milestone 9 — Inheritance and Mutation

Binary fission now becomes evolutionary.

Initially:

```text
Parent genome
      ↓
copy
      ↓
small mutation
      ↓
Child genome
```

Example:

```text
Parent
ftsZ = 0.520

Child
ftsZ = 0.527
```

or:

```text
ftsZ = 0.514
```

Mutations should normally be small.

Clamp gene activity:

```text
0.0 <= activity <= 1.0
```

## Mutation rules

Start with:

```text
mutation chance
mutation magnitude
```

Keep these global initially.

Later they may themselves become heritable traits.

### Done when

- [ ] Children usually resemble parents
- [ ] Some offspring differ slightly
- [ ] Gene values remain valid
- [ ] Mutation is deterministic under the simulation seed
- [ ] Different lineages begin to appear

---

# 17. Milestone 10 — Natural Selection

At this stage do **not** explicitly code:

```text
if gene is good:
    reproduce more
```

Selection should emerge from the simulation.

Example:

```text
Gene A
→ moves faster
→ reaches nutrients
→ spends more energy

Gene B
→ moves slower
→ uses less energy
```

In a nutrient-rich world, A may win.

In a nutrient-poor world, B may win.

That is the desired system.

### Done when

- [ ] Different gene values produce measurable trade-offs
- [ ] Some lineages reproduce more successfully
- [ ] Population averages change across generations
- [ ] The environment changes which traits perform well

---

# 18. Milestone 11 — Lineages and Generations

Give every organism:

```text
id
parent_id
generation
```

Optional later:

```text
lineage_id
```

This allows:

```text
Organism 1
├── Organism 2
│   ├── Organism 4
│   └── Organism 5
└── Organism 3
    └── Organism 6
```

Track:

- population
- births
- deaths
- generation
- average energy
- average gene values
- oldest lineage
- lineage size

### Done when

- [ ] Parent-child relationships are recorded
- [ ] Generations can be inspected
- [ ] Gene changes can be followed over time

---

# 19. Milestone 12 — Simulation Inspector

Extend the current tile inspector.

When hovering an organism:

```text
ORGANISM #37

Age         18.4
Energy      63.2
Generation  7

GENOME
ftsZ        0.538
```

Later:

```text
motility    ...
stress      ...
metabolism  ...
```

Allow toggling between:

```text
Tile Inspector
Organism Inspector
```

This becomes essential once evolution starts.

---

# 20. Milestone 13 — Population Statistics

Add a small simulation panel.

Example:

```text
Population       184
Births           921
Deaths           738
Max generation    19

Mean ftsZ       0.573
```

Later add graphs for:

```text
population vs time
mean gene value vs time
resource level vs time
lineage frequencies
```

Do this only after the underlying simulation is stable.

---

# 21. Milestone 14 — Different Bacterial Profiles

Only after several gene systems work should the project attempt different bacteria.

Avoid:

```rust
enum Species {
    EColi,
    SpeciesB,
    SpeciesC,
}
```

as the primary behaviour system.

Prefer:

```text
Bacterial profile
      ↓
starting genome values
+
environmental preferences
+
physiological parameters
```

Then a profile initializes organisms, but the normal genome/phenotype simulation still controls them.

Example concept:

```text
Profile A
├── division activity
├── motility activity
├── stress response
└── metabolic preference

Profile B
├── division activity
├── motility activity
├── stress response
└── metabolic preference
```

A named real bacterium should be added only when the relevant parameters are backed by reliable biological sources.

The simulation should label these as **approximations**, not literal reproductions of real strains.

---

# 22. Milestone 15 — Competition Between Populations

Spawn two starting genomes.

Example:

```text
Population A
ftsZ = 0.42
...

Population B
ftsZ = 0.70
...
```

Both live in the same world and compete for resources.

Observe:

```text
population size
resource use
survival
gene distribution
```

Do not artificially decide the winner.

### Done when

- [ ] Two lineages coexist
- [ ] They use the same resource system
- [ ] Population proportions change naturally
- [ ] Different environments produce different outcomes

---

# 23. Milestone 16 — Environmental Stress

Use the world you already generated.

Possible environmental variables:

```text
moisture
terrain
resource availability
local crowding
```

Later additional fields can be introduced only when needed.

Organisms may develop different tolerances through their genomes.

This creates environmental niches.

Example:

```text
wet region
→ lineage A performs well

dry region
→ lineage B survives better
```

Now world generation begins directly influencing evolution.

---

# 24. Milestone 17 — Short-Term State / Memory

The original Procedural Ecosystem idea included memory.

For bacteria, do not begin with human-like memory.

Use a small adaptive state such as:

```text
recent nutrient trend
recent movement direction
recent stress
previous resource concentration
```

Example:

```text
resource getting better
→ continue direction

resource getting worse
→ increase chance of changing direction
```

This keeps the old memory concept while fitting the microbial model better.

---

# 25. Milestone 18 — Weighted Decisions

Once sensing and short-term state exist:

```text
possible actions
├── continue
├── turn
├── slow
├── move
└── remain
```

Each receives a weight based on:

```text
energy
resource gradient
stress
gene-derived phenotype
recent state
randomness
```

Then choose probabilistically.

This preserves emergent behaviour without hard-coded scripts.

---

# 26. Milestone 19 — Ecosystem Expansion

Only after one microbial population evolves successfully:

Possible extensions:

```text
multiple nutrient types
competition
cooperation
secreted resources
toxic by-products
environment modification
colonies
biofilm-like clustering
specialized metabolic niches
```

Add one system at a time.

The ecosystem should become richer through interactions, not through a long list of manually scripted species rules.

---

# 27. Milestone 20 — Second Species / Predator-Prey Branch

The original ecosystem roadmap eventually introduced a second species and predator-prey dynamics.

For the microbial version, first implement:

```text
two competing microbial populations
```

Only later consider a separate antagonistic/predatory system.

Possible future branches:

```text
predatory microbe
phage-like agent
toxin-producing competitor
resource specialist
```

This is late-stage work.

---

# 28. Milestone 21 — Optional Evolved Controller

The original Procedural Ecosystem idea eventually allowed a tiny evolved neural network.

Keep this as an **optional experimental branch**, not part of the core bacterial model.

Possible late-stage controller:

```text
Inputs
├── nutrient gradient
├── energy
├── stress
├── local density
└── recent state

Tiny controller

Outputs
├── turn
├── movement intensity
└── behavioural state
```

Its weights could mutate and evolve.

This is a computational evolution experiment, not a claim that bacteria contain neural networks.

Do not begin this until the gene-based simulation already works.

---

# 29. Core Evolution Loop

The final core loop should eventually look like:

```text
                 ┌───────────────────────┐
                 │      Environment      │
                 │ terrain / moisture    │
                 │ nutrients / stress    │
                 └───────────┬───────────┘
                             │
                             ↓
                    ┌────────────────┐
                    │    Organism    │
                    │ genome + state │
                    └───────┬────────┘
                            │
             ┌──────────────┼──────────────┐
             ↓              ↓              ↓
          sensing        metabolism      movement
             │              │              │
             └──────────────┼──────────────┘
                            ↓
                         energy
                            ↓
                    survive / divide
                            ↓
                         offspring
                            ↓
                         mutation
                            ↓
                       new genome
                            ↓
                       next cycle
```

---

# 30. Development Rules

## Rule 1 — One system at a time

Do not implement:

```text
movement + mutation + neural network + predators
```

in one milestone.

---

## Rule 2 — Always make the previous system observable

Before adding mutation, make reproduction visible.

Before adding selection, make mutations inspectable.

Before adding multiple species, make lineage statistics visible.

---

## Rule 3 — Determinism matters

Given:

```text
same world seed
same simulation seed
same starting genome
```

the simulation should ideally be reproducible.

Randomness should come from controlled seeded RNG rather than uncontrolled random calls once evolution begins.

---

## Rule 4 — Genes need trade-offs

Avoid:

```text
higher gene value = universally better
```

Prefer:

```text
benefit
+
cost
```

Selection only becomes interesting when environments reward different strategies.

---

## Rule 5 — Separate genotype from phenotype

Do not scatter gene checks everywhere.

Prefer:

```text
Genome
   ↓
Phenotype
   ↓
Simulation behaviour
```

For example:

```text
ftsZ activity
   ↓
division traits
   ↓
division code
```

---

## Rule 6 — Do not over-model biology immediately

The project should first be a coherent evolutionary simulation.

Increase biological fidelity after:

```text
birth
death
resources
inheritance
mutation
selection
```

all work together.

---

# 31. Immediate Implementation Order

This is the exact order to work from the current codebase.

```text
[1] Create organism.rs
 ↓
[2] Create one Organism
 ↓
[3] Add Genome with ftsZ
 ↓
[4] Spawn exactly one organism
 ↓
[5] Render it as a circle
 ↓
[6] Add age
 ↓
[7] Add energy + metabolism
 ↓
[8] Add death
 ↓
[9] Add ftsZ-dependent binary fission
 ↓
[10] Confirm 1 → 2 → 4 organisms
 ↓
[11] Add temporary energy gain
 ↓
[12] Stress-test reproduction
 ↓
[13] Replace temporary energy with nutrients
 ↓
[14] Add simple movement
 ↓
[15] Add environmental sensing
 ↓
[16] Add inheritance
 ↓
[17] Add mutation
 ↓
[18] Add lineage tracking
 ↓
[19] Observe natural selection
 ↓
[20] Add second gene
```

Do not jump to step 20 before steps 1–19 form a complete stable loop.

---

# 32. First Major Target

## IcyLife v0.1 — One Replicating Cell

Success means:

```text
One cell exists in the procedural world.

It:
- has energy
- ages
- has an ftsZ gene value
- consumes energy
- gains energy from its environment
- divides when conditions are satisfied
- produces a child
- passes its genome to that child
- can die
```

No mutation required yet.

---

# 33. Second Major Target

## IcyLife v0.2 — Heritable Variation

Success means:

```text
offspring inherit genomes
+
small mutations occur
+
gene values affect phenotype
+
different descendants behave differently
```

At this point the project has actual evolutionary potential.

---

# 34. Third Major Target

## IcyLife v0.3 — Selection

Success means:

```text
resources are limited
+
organisms compete
+
different gene values have trade-offs
+
some lineages reproduce more successfully
+
population gene distributions change over time
```

This is the first version where natural selection can genuinely emerge from the simulation.

---

# 35. Fourth Major Target

## IcyLife v0.4 — Procedural Microbial Ecosystem

Success means:

```text
multiple genes
+
movement
+
environment sensing
+
environmental niches
+
multiple lineages
+
resource competition
+
inspectable evolutionary history
```

At this point, new bacterial profiles and more biologically informed parameter sets become worthwhile.

---

# 36. Long-Term Direction

The project can eventually become:

```text
Seed
 ↓
Procedural environment
 ↓
Starting bacterium/genome
 ↓
Population growth
 ↓
Resource competition
 ↓
Mutation
 ↓
Selection
 ↓
Lineage divergence
 ↓
Different ecological strategies
 ↓
Emergent microbial ecosystem
```

The key principle is:

> Do not procedurally generate creatures only by appearance. Procedurally generate the **rules they inherit**, then let the environment determine which rules survive.
