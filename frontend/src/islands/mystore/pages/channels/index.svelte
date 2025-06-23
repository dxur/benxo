<script lang="ts">
  import Table from "../../components/Table.svelte";
  import TablePagination from "../../components/TablePagination.svelte";
  import LoadingShow from "../../components/LoadingShow.svelte";
  import Dialog from "../../components/Dialog.svelte";
  import * as ApiRoutes from "@bindings/ApiRoutes";
  import { writable } from "svelte/store";
  import { useNavigate } from "@dvcol/svelte-simple-router";
  import { AppRoutes } from "../../routes";
  import type { Page } from "@bindings/Page";
  import type { ChannelCreate } from "@bindings/ChannelCreate";
  import type { LoadingStatus } from "../../components/LoadingShow.svelte";
  import { notifCenter } from "@/stores/notifications";

  const { push } = useNavigate();

  interface Channel {
    id: string;
    type: "STORE" | "API" | "CUSTOM";
    name: string;
    active: boolean;
    creation_date: string;
    update_date: string;
  }

  let page = writable(1);
  let total = writable(1);
  let dialog: HTMLDialogElement;
  let status: LoadingStatus = undefined;
  let channels: Page<Channel>;

  let fields = default_fields();

  function default_fields() {
    return {
      type: "",
      name: "",
    };
  }

  function pull(p: number) {
    channels = {
      data: [
        {
          id: "6828c11ef6c3689ebd35dee0",
          type: "STORE",
          name: "Main Store",
          active: true,
          creation_date: "2022-04-31",
          update_date: "2022-04-31",
        },
        {
          id: "6828c11ef6c3689ebd35dee1",
          type: "API",
          name: "Woocommerce API",
          active: false,
          creation_date: "2022-05-01",
          update_date: "2022-05-01",
        },
        {
          id: "6828c11ef6c3689ebd35dee2",
          type: "CUSTOM",
          name: "Marketplace A",
          active: true,
          creation_date: "2022-06-15",
          update_date: "2022-06-15",
        },
      ],
      total: 3,
      page: 1,
      per_page: 10,
    };
    status = null;
  }

  function edit(type: string, id: string) {
    console.log(id);
    switch (type) {
      case "STORE":
        push({
          path: AppRoutes.CHANNEL_STORE.path,
          params: { oid: id },
        });
        break;
      case "API":
        push({
          path: AppRoutes.CHANNEL_API.path,
          params: { oid: id },
        });
        break;
      case "CUSTOM":
        notifCenter.warning("Not implemented yet");
        break;
    }
  }

  function create() {
    console.log(fields.name, fields.type);
    ApiRoutes.create_channel({
      name: fields.name,
      channel: "Custom",
    })
      .then((c) => {
        notifCenter.success("Channel created successfully");
        edit("Custom", c.name);
      })
      .catch((err) => notifCenter.error(err));
  }

  function remove(id: string) {}

  $: pull($page);
  $: document.title = "Channels";
</script>

<LoadingShow {status}>
  <header>
    <h1>Channels</h1>
    <button
      on:click={() => {
        fields = default_fields();
        dialog.showModal();
      }}
    >
      New
    </button>
  </header>

  {#if channels.data.length}
    <Table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Type</th>
          <th>Active</th>
          <th>Creation Date</th>
          <th>Update Date</th>
          <th>Options</th>
        </tr>
      </thead>
      <tbody>
        {#each channels.data as channel}
          <tr>
            <td>{channel.name}</td>
            <td>{channel.type}</td>
            <td>{channel.active ? "Yes" : "No"}</td>
            <td>{channel.creation_date}</td>
            <td>{channel.update_date}</td>
            <td>
              <button on:click={() => edit(channel.type, channel.id)}
                >Edit</button
              >
              <button type="reset" on:click={() => remove(channel.id)}
                >Delete</button
              >
            </td>
          </tr>
        {/each}
      </tbody>
    </Table>
    <TablePagination {page} {total} />
  {:else}
    <span>Oops nothing here!</span>
  {/if}

  <Dialog bind:dialog>
    <header>
      <h2>Create Channel</h2>
    </header>
    <form
      on:submit={(ev) => {
        ev.preventDefault();
        create();
      }}
    >
      <fieldset>
        <label>
          Name
          <input type="text" bind:value={fields.name} required />
        </label>
      </fieldset>
      <fieldset>
        <label>
          Type
          <select bind:value={fields.type} required>
            <option value="store">Store</option>
            <option value="api">API</option>
            <option value="marketplace">Marketplace</option>
          </select>
        </label>
      </fieldset>
      <button type="submit">Submit</button>
      <button type="button" on:click={() => dialog.close()}>Close</button>
    </form>
  </Dialog>
</LoadingShow>
