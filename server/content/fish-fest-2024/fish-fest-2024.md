# The Making of Fishing for a Princess

I've always liked foddian games, the simple controls and mechanics make these
easy to learn but take time to master and complete. They are skill-based where
the amount of time you play them directly influences how good you are at the
game. Games like this are of course the hit title _Getting Over It with Bennett
Foddy_ which reintroduced the genre back in 2017, _Pogostuck_ with its
multiplayer climbing and _Jump King_ which has my favourite style between
these.

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
[Bevy](https://bevyengine.org/) and by getting a fish placeholder
asset. _Bevy_ is an Entity-Component-System engine, meaning everything fits
into this structure where an Entity has Components and Systems update those
components. I very much enjoy coding this way, it lets me group related
systems and components into the same file. For example, all the multiplayer
ECS is inside the multiplayer.rs file.

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
to explain with an image, this one by
[@FreyaHomer](https://www.youtube.com/@Acegikmo) gives a good explanation of
it.

![Bevy Coordinate System](/public/images/fish-fest-2024/handedness.webp)

One thing I've learnt from my previous game dev attempts is that you should
never write your own physics, so I decided to use a library that has direct
bevy support [Rapier](https://rapier.rs/). To plug it in, all I have to do
is add the plugin it exposes.

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
- No animations lol

Maia had drawn some fish for me, so I put it in the game and...

<iframe src="https://www.youtube-nocookie.com/embed/DNBnluRCflU?si=mjrMEq-ZtPksP0Z7"
    width="560" height="315"
    title="YouTube video player" frameborder="0"
    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
    referrerpolicy="strict-origin-when-cross-origin"
    allowfullscreen>
</iframe>

Fish too big. I fixed that dw 😉.

Up to this point, I had hard coded the map collider platforms and walls. If I
was going to make this available to the community I needed to fashion up some
editor.

## Loading a real map

I started developing the map editor on the last day of the game jam, I actually
[live-streamed](https://youtu.be/pBtzOZ-Odz4) making the map editor in full but
the plan was that the editor would be a separate application to create maps
where you can load a background image and place platforms and walls on top of
it. The background should already include the platforms drawn to make it easier
to load. Easier to show an image of my plan shown on stream around
[18:19](https://youtu.be/pBtzOZ-Odz4?t=1099).

![Fish Editor Plan](/public/images/fish-fest-2024/fish_editor_plan.webp)

Using [egui](https://www.egui.rs/) I could load an image and create menus to
add colliders and objects. I then defined my map file structure.

```rust
pub struct MapFile {
    // The relative or absolute path to the image file
    pub image_path: String,
    // This lets our editors load any sized image and scale it down to work
    // better on the game
    pub image_scale: f32,
    // Vec<> are lists of objects in rust
    // This one contains all the platforms and walls
    pub colliders: Vec<Collider>,
    // This one contains map objects like player spawn
    pub objects: Vec<MapObject>,
}

pub struct Collider {
    pub collision: Collision,
    pub rect: Rect,
}

pub enum Collision {
    Platform,
    Wall,
}

// A rectangle storing an initial position and a final position
pub struct Rect {
    pub min: Vec2<f32>,
    pub max: Vec2<f32>,
}

// This is a position struct, a Vector 2D with an x and y position. x and y can
// be any type, most of the time a f32 (32-bit float) for precise position
// control
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

pub struct MapObject {
    pub pos: Vec2<f32>,
    pub object: MapObjectType,
}

pub enum MapObjectType {
    PlayerSpawn,
    PrincessSpawn,
}
```

With this structure complete I started making the map editor. Egui is an
immediate-mode renderer. This means the UI updates every frame the window is
painted, this makes it really easy to manage state, at least for me.
Here's an example directly from their [README](https://github.com/emilk/egui)
file.

```rust
use egui;
fn render(ctx: &egui::Context, name: &mut String, age: &mut i32) {
    egui::Window::new("My Window").show(ctx, |ui| {
        ui.heading("My egui Application");
        ui.horizontal(|ui| {
            ui.label("Your name: ");
            ui.text_edit_singleline(&mut name);
        });
        ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
        if ui.button("Increment").clicked() {
            age += 1;
        }
        ui.label(format!("Hello '{name}', age {age}"));
        ui.image(egui::include_image!("ferris.png"));
    });
}
```

![Image from the example code in egui README](https://github.com/emilk/egui/blob/master/media/demo.gif?raw=true)

The above code won't actually render anything since egui is a rendering library
and does not have any window creation utilities. The winit (window initializer)
crate provided by egui developers is
[eframe](https://github.com/emilk/egui/tree/master/crates/eframe) that will
manage the control loop and lifetime of the application.

After initializing the window and creating a state to hold all the colliders I
had created an editor only a mother would love.

![Map Editor Draft](/public/images/fish-fest-2024/map_editor.webp)

Level files are saved to a `.ron` file format, a file format similar to _JSON_
but made for _Rust_ which stands for _Rusty-Object-Notation_.

I asked the best artist in the world to make me a test map, so I could try my
editor out.

![Test Map Background](/public/images/fish-fest-2024/test_map.webp)

With this map I produced this `.ron` file. (Unfortunately the library I use to
syntax highlight code on this blog does not support ron syntax at the time of
writing).

```ron
(
    image_path: "./test.png",
    image_scale: 0.3,
    colliders: [
        (
            collision: Platform,
            rect: (
                min: (
                    x: -16.957031,
                    y: -16.3125,
                ),
                max: (
                    x: -6.9570313,
                    y: -6.3125,
                ),
            ),
        ),
        // ...
    ],
    objects: [
        (
            pos: (
                x: 370.5625,
                y: 264.29688,
            ),
            object: PlayerSpawn,
        ),
        // ...
    ],
)
```

To load the map, I coded up some map loading function that would interpret the
rectangles and positions on this ron file and spawn entities with colliders and
rigid bodies. I'm skipping over these things but if you're more interested in
this type of stuff I go all over it on 
[my stream](https://youtu.be/pBtzOZ-Odz4), most of the process of making this
editor is on there. Loading my test map I ended up with this.

<iframe src="https://www.youtube-nocookie.com/embed/EFW5W8PPH94?si=xXVyTZXqtAcWDbDv"
    width="560" height="315"
    title="YouTube video player" frameborder="0"
    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
    referrerpolicy="strict-origin-when-cross-origin"
    allowfullscreen>
</iframe>

I don't know if you can see very well but on that video the time is **22:05**,
with 2 hours left and nerves of steel, me and Maia tried our best to smooth the
edges of our game before posting it to the Game Jam.

## One hour before deadline

Before making the map, my plan showed slopes and a princess. I never got to
implementing those neither on the editor nor the game. I also wanted to
implement multiplayer but with 1 hour left all I could do was kill some bugs
while I asked Maia to make the final map.

Below is the developer speedrun from the version that was submitted to the game
jam.

<iframe src="https://www.youtube-nocookie.com/embed/BW6Of2w7y-4?si=hqpQjBxc60FOw1Jt"
    width="560" height="315"
    title="YouTube video player" frameborder="0"
    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
    referrerpolicy="strict-origin-when-cross-origin"
    allowfullscreen>
</iframe>

We submitted the game with about 3 minutes left of the jam? We finished 499th
out of 587 submissions, but we ended with our most complete product yet!

The game is available on itch.io for Windows, or Linux. You can get it from
the link below to give it a try, most likely a more recent version of the game.

<iframe frameborder="0" src="https://itch.io/embed/2634128?bg_color=ffffff"
    width="552" height="167">
    <a href="https://onelikeandidie.itch.io/fishing-for-a-princess">
        Fishing for a Princess by onelikeandidie
    </a>
</iframe>

## Sticking to the plan, setting up multiplayer

So what happens after a jam? Most of the time, the plan made in a jam is never
completed and the product never got its shape but with our first playable game
me and Maia set out to conquer multiplayer support.

One of the challenges of adding multiplayer support is making sure to send not
only the smallest amount of data possible but also only the most important
data. Competitive games like _Counter Strike_ and _League of Legends_ send all
player positions every millisecond so that you have the most up-to-date
information so that you as a player can make plays and counter-plays as fast
as possible.

My game is not a competitive game, well at least in my eyes it's a more casual
game to enjoy raging with your friends, so I decided to take another approach
to making the game's networking. I decided I was going to send the information
when players jump and do other actions instead of constantly sending player
position. This way each player simulates their view of the level and gets
information to update the other players in their lobby.

The idea is that the players connect to a central server and that server then
relays the info back around to each client, basically peer-to-peer but the
clients don't have each other's real IPs. The network stack is in UDP, I
decided to use UDP since it's not too bad if we lose some packets to the void
since it's not a competitive game and the packets are smaller. Competitive
games normally use TCP because the packets are always send and received in
order ensuring consistency between server and client. To counteract the
possible desync between the server and client I also made sure to once in a
while send a packet containing the player's current position to make sure
everything is synced up properly.

I'm using a crate called [Laminar](https://crates.io/crates/laminar) to host
the server and connect the clients and the
[bincode](https://crates.io/crates/bincode) crate to serialize the packet data into
bytes and back. Here's a code example.

```rust
// Serde is to help serialize and deserialize game packets
use serde::{Serialize, Deserialize};
// Laminar is to create UDP sockets
use laminar::{Socket, Packet};
use std::time::Instant;

// Our game packet structure
#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum GamePacket {
    Jump(f32),
    Sync(f32, f32),
}

fn client() {
    let server_addr = "127.0.0.1:12346";
    // Creates the client socket
    let mut socket = Socket::bind("127.0.0.1:12345").unwrap();
    let packet_sender = socket.get_packet_sender();
    // Create a jump packet
    let game_packet = GamePacket::Jump(10.0);
    let encoded_packet = Packet::unreliable(
        server_addr,
        bincode::serialize(&game_packet)
    );
    // Send the packet to the server
    packet_sender.send(encoded_packet).unwrap();
    socket.manual_poll(Instant::now());
}

fn server() {
    let mut socket = Socket::bind("127.0.0.1:12346").unwrap();
    // Setup receiving packets
    let event_receiver = socket.get_event_receiver();
    // Setup sending packets
    let packet_sender = socket.get_packet_sender();
    // Start polling on a separate thread
    let _thread = thread::spawn(move || socket.start_polling());
    // Receive a packet, replace with for loop for repeatedly receive packets
    let socket_event = event_receiver.recv().unwrap();
    match socket_event {
        // Packets are received here
        SocketEvent::Packet(packet) => {
            let endpoint: SocketAddr = packet.addr();
            let received_data: &[u8] = packet.payload();
            // Decode the packets
            let decoded_packet: GamePacket = bincode::deserialize(received_data).unwrap();
            // Do something with the packets...
        }
        SocketEvent::Connect(connect_event) => { /* a client connected */ }
        SocketEvent::Timeout(timeout_event) => { /* a client timed out */ }
        SocketEvent::Disconnect(disconnect_event) => { /* a client disconnected */ }
    }
}
```

To code the client I had to make 2 bevy systems, one to poll for packets
manually since I don't want to pass the socket into another thread and the
other system to process the received packets. This took quite a long time to
code up, I went through a couple of issues but basically each client gets a
random id and the server puts them all in a list along with their IP to know
which clients to send the packets to.

I encountered a lot of bugs on the way, sometimes clients would send their
connection data over and over again, send their synchronization packets
repeatedly or not sending jump packets for the uncontrolled jumps... Here's an
easy peek into my bugs, while testing with multiple clients they got confused
on which is which.

<iframe src="https://www.youtube-nocookie.com/embed/6QuaQjD9As8?si=ZfIQCxNE0cS8n_6t"
    width="560" height="315"
    title="YouTube video player" frameborder="0"
    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
    referrerpolicy="strict-origin-when-cross-origin"
    allowfullscreen>
</iframe>

<iframe src="https://www.youtube-nocookie.com/embed/LYcC8o4xFqE?si=JqAZGy9BaVC8y21t"
    width="560" height="315"
    title="YouTube video player" frameborder="0"
    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
    referrerpolicy="strict-origin-when-cross-origin"
    allowfullscreen>
</iframe>

I got a few of my friends to rage at the game with me and most of them quit
after the first few jumps.

<iframe src="https://www.youtube-nocookie.com/embed/CX0PyHM9qCg?si=VdkWuSnfGsOzYe-3"
    width="560" height="315"
    title="YouTube video player" frameborder="0"
    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
    referrerpolicy="strict-origin-when-cross-origin"
    allowfullscreen>
</iframe>

That's it for now, I'm planning many more updates to make this a more complete
game. Here's a simple roadmap for the next few updates I'm working on with
Maia. I'll see you next time when I have time to make some more posts.

- 0.4.0 - Map Improvements
  - I want to have more map creation options to make more complex and difficult
  maps
- 0.5.0 - New Online Features
  - A couple more online features to make it a little more interesting even
  when you're not directly playing with other players
- 0.6.0 - Map Scripting and Community Support
  - Add basic and advanced scripting abilities and a site to share maps

🐟 👋

---

_This article was updated on the 6th May 2024 to fix some typos and some of the
wording._
