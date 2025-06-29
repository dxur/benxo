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
  import type { LoadingStatus } from "../../components/LoadingShow.svelte";
  import { notifCenter } from "@/stores/notifications";
  import type { StorePublic } from "@bindings/StorePublic";

  const { push } = useNavigate();

  let page = writable(1);
  let total = writable(1);
  let dialog: HTMLDialogElement;
  let status: LoadingStatus = undefined;
  let stores: Page<StorePublic>;

  let fields = default_fields();

  function default_fields() {
    return {
      store_id: "",
      name: "",
    };
  }

  function pull(p: number) {
    ApiRoutes.get_all_stores({
      page: p,
      per_page: null,
    })
      .then((data) => {
        stores = data;
        total.update((_) => Math.ceil(data.total / data.per_page));
        status = null;
      })
      .catch((err) => {
        notifCenter.error(err);
        status = err || "An error occurred";
      });
  }

  function edit(id: string) {
    push({
      path: AppRoutes.STORE.path,
      params: { oid: id },
    });
  }

  function create() {
    console.log(fields.store_id, fields.name);
    ApiRoutes.create_store({
      store_id: fields.store_id,
      name: fields.name,
    })
      .then((res) => {
        notifCenter.success("Store created successfully");
        edit(res.store_id);
      })
      .catch((err) => notifCenter.error(err));
  }

  function remove(store_id: string) {
    ApiRoutes.delete_store({ store_id })
      .then(() => {
        notifCenter.success("Product deleted");
        pull($page);
      })
      .catch((err) => notifCenter.error(err));
  }

  $: pull($page);
  $: document.title = "Stores";
</script>

<LoadingShow {status}>
  <header>
    <h1>Stores</h1>
    <button
      on:click={() => {
        fields = default_fields();
        dialog.showModal();
      }}
    >
      New
    </button>
  </header>

  {#if stores.data.length}
    <Table>
      <thead>
        <tr>
          <th>Store ID</th>
          <th>Name</th>
          <th>Options</th>
        </tr>
      </thead>
      <tbody>
        {#each stores.data as store}
          <tr>
            <td>{store.store_id}</td>
            <td>{store.name}</td>
            <td>
              <button on:click={() => edit(store.store_id)}>Edit</button>
              <button>
                <a
                  target="_blank"
                  href={`https://${store.store_id}.mystore.benxo.test`}
                  >Preview</a
                >
              </button>
              <button type="reset" on:click={() => remove(store.store_id)}
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
          Store Id
          <input type="text" bind:value={fields.store_id} required />
        </label>
      </fieldset>
      <fieldset>
        <label>
          Name
          <input type="text" bind:value={fields.name} required />
        </label>
      </fieldset>
      <button type="submit">Submit</button>
      <button type="button" on:click={() => dialog.close()}>Close</button>
    </form>
  </Dialog>
</LoadingShow>
