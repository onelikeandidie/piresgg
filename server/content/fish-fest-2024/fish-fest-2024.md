# The Making of Fishing for a Princess

I've always liked foddian games, the simple controls and mechanics make these
games easy to learn but take time to master and complete. They are skill-based
games where the amount of time you play the game directly influences how good
you are at the game. Games like this are of course the hit title foddian game
_Getting Over It with Bennett Foddy_ which reintroduced the genre back in 2017,
_Pogostuck_ with its multiplayer climbing and _Jump King_ which has my
favourite style between these.

Although I've never completed _Pogostuck_ or _Jump King_ these are my favourite
games out of the bunch, I love the rageful climb with my friends in _Pogostuck_
but love the simplicity of _Jump King_ so when I saw the "Fish Fest 2024"
had no theme requirements I jumped with excitement. So I grabbed the best
artist I know and hatched up a plan.

## First Jump: Inspiration

I always envied the simplicity of _Jump King_'s control scheme, although it
makes for a frustrating experience with no visual jump indicator, you just have
to get accustomed to the jumping mechanics and timing, it rewards the timesink
into the game to try and beat it.

Because of _Jump King_'s popularity, there  was also a few community maps
created like _Babe of Ascension_ and the other _Babe of `title here`_ which
required the use of a mod to make work. Recently _Jump King_ has added support
for the Steam Workshop for sharing maps but why  not have this feature from the
start? The tools you used to make the game  should be available for the
community to make maps as well. So the first ingredient to making Fishing for
a Princess was found.

When you play _Pogostuck_, you will randomly see players who have similar
progress to you jump around you. I've always found it more enjoyable to see
others fail alongside me, it gave me motivation to keep going and succeed.
Joining my friends always makes it more fun to play. So that was my second
ingredient, multiplayer.

## A rough first shape

I started experimenting with my favourite game engine
[Bevy](https://bevyengine.org/) and started by getting a fish placeholder
asset. _Bevy_ is an Entity-Component-System engine, meaning everything fits
into this structure where an Entity has Components and Systems update those
components. I like this because it keeps me from writing everything in one
place.

It's quite simple to understand, let me give you a quick code introduction.

```rust
// Import bevy
use bevy::prelude::*;

// Structure our components

// A position keeps an x and y numeric values
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
// Player lets us filter the Entities by the player
#[derive(Component)]
struct Player;

// This is a system, we can query and ask for resources just by putting the
// type of the system in the function parameters.
fn spawn_player(mut commands: Commands) {
    commands.spawn(Player).insert(Position { x: 0.0, y: 0.0 });
}

// Using a query type, we can ask Bevy to give us the components we want to
// change or use for our system.
fn move_player_right(mut query: Query<&mut Position, With<Player>>) {
    // Go through every player and move them left
    for mut position in &mut query {
        position.x += 1.0;
    }
}

// This time, we don't use a &mut because we don't want to modify the Position
fn print_player_position(query: Query<&Position, With<Player>>) {
    // Go through every player and print their position
    for position in &query {
        println!("Player x:{:?} y:{:?}", position.x, position.y);
    }
}

// We start our game and set up our systems here
fn main() {
    App::new()
        // Systems can be bundled into a plugin, this one comes with bevy to
        // set up the game loop, render and listen for key presses
        .add_plugins(DefaultPlugins)
        // Here we add the system to run when the game starts so that we can
        // spawn the player
        .add_systems(Startup, spawn_player)
        // Running a system on the Update set will make it run on every frame
        .add_systems(Update, move_player_right)
        // We can't see our player since there's no Sprite component so a
        // system to print this out is required
        .add_systems(Update, print_player_position)
        .run();
}
```

Before attempting to run this code, add ``bevy`` as a dependency with ``cargo
add bevy`` or copying from [crates.io](https://crates.io/crates/bevy). This
example worked with ``bevy = "0.13.2"``. After running, you should be able to
see a window open and your terminal will spam the player's position constantly
increasing on the x coordinate.

Bevy's coordinate system is different from Unity and Unreal engine, it's easier
to explain with an image, I found this one before by
[@FreyaHomer](https://www.youtube.com/@Acegikmo) which shows better how it
works.

![Bevy Coordinate System](/public/images/fish-fest-2024/handedness.webp)

One thing I've learnt from my previous game dev attempts is that you should
never write your own physics, so I decided to use a physics library that has
direct bevy support [Rapier](https://rapier.rs/). To plugin it in, all I have
to do is add the plugin it exposes.

```rust
// ...
fn main() {
    App::new()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // Adding this extra plugin will let us see hit boxes without sprites
        .add_plugins(RapierDebugRenderPlugin::default());
}
// ...
```

And adding physics is as simple as adding Colliders and a RigidBody to the
Player.

```rust
fn spawn_player(mut commands: Commands) {
    commands.spawn(Player)
        // I've replaced this with Bevy's actual position component as it
        // doesn't work with our custom one without adding complexity
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(50.0));
}
```

Skipping over a couple of details like loading sprites, hard coding platforms
and handling input I arrived at my first draft.

<iframe src="https://www.youtube-nocookie.com/embed/pwzNS051-gY?si=Sns_4tPoUy4otqjO"
    width="560" height="315"
    title="YouTube video player" frameborder="0"
    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
    referrerpolicy="strict-origin-when-cross-origin"
    allowfullscreen>
</iframe>

The player fish has a couple of features:

- The longer you hold space, the further it will jump
- Not pressing any buttons will lead the fish to flop around every couple of
    seconds

## Loading a real map



<iframe src="https://www.youtube-nocookie.com/embed/BW6Of2w7y-4?si=hqpQjBxc60FOw1Jt"
    width="560" height="315"
    title="YouTube video player" frameborder="0"
    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
    referrerpolicy="strict-origin-when-cross-origin"
    allowfullscreen>
</iframe>

## Where can you find it?

The game is available on itch.io for Windows, or Linux. You can get it from
the link below.

<iframe frameborder="0" src="https://itch.io/embed/2634128?bg_color=ffffff"
    width="552" height="167">
    <a href="https://onelikeandidie.itch.io/fishing-for-a-princess">
        Fishing for a Princess by onelikeandidie
    </a>
</iframe>
