<script lang="ts">
    import { auth, me } from "@bindings/UserRoutes";

    let email = $state("");
    let password = $state("");

    $effect(() => {
        me().then(() => {
            console.error("Logout first");
            location.href = "/user-center";
        });
    });

    function login(e: Event) {
        e.preventDefault();
        auth({
            Login: {
                email,
                password,
            },
        })
            .then((_) => {
                location.href = "/user-center";
            })
            .catch((e) => {
                alert(e);
            });
    }
</script>

<form onsubmit={login}>
    <input bind:value={email} type="email" name="email" placeholder="Email" />
    <input
        bind:value={password}
        type="password"
        name="password"
        autocomplete="on"
        placeholder="Password"
    />
    <button>Login</button>
</form>
