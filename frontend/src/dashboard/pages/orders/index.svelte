<script lang="ts">
  import Table from "@/dashboard/components/Table.svelte";
  import TablePagination from "@/dashboard/components/TablePagination.svelte";
  import LoadingShow from "@/dashboard/components/LoadingShow.svelte";
  import Dialog from "@/dashboard/components/Dialog.svelte";
  import * as ApiRoutes from "@bindings/ApiRoutes";
  import { writable } from "svelte/store";
  import { useNavigate } from "@dvcol/svelte-simple-router";
  import { AppRoutes } from "@/dashboard/routes";
  import { getOid } from "@/dashboard/lib/utils";
  import { notifCenter } from "@/dashboard/stores/notifications";
  import type { Page } from "@bindings/Page";
  import type { OrderPublic } from "@bindings/OrderPublic";
  import type { DeliveryType } from "@bindings/DeliveryType";
  import type { LoadingStatus } from "@/dashboard/components/LoadingShow.svelte";

  const { push } = useNavigate();

  let page = writable(1);
  let total = writable(1);
  let dialog: HTMLDialogElement;
  let status: LoadingStatus = undefined;
  let orders: Page<OrderPublic>;

  let fields = default_fields();

  function default_fields() {
    return {
      full_name: "",
      phone: "",
      email: "",
      province: "",
      address: "",
      delivery: "STOP_DESK" as DeliveryType,
      note: "",
      items: {},
    };
  }

  function pull(p: number) {
    ApiRoutes.get_all_orders({
      page: p,
      per_page: null,
    })
      .then((data) => {
        orders = data;
        total.update((_) => Math.ceil(data.total / data.per_page));
        status = null;
      })
      .catch((err) => {
        notifCenter.error(err);
        status = err;
      });
  }

  function edit(id: string) {
    push({
      path: AppRoutes.ORDER.path,
      params: { oid: getOid(id) },
    });
  }

  function create() {
    ApiRoutes.create_order(fields)
      .then((p) => {
        notifCenter.success("Order created successfully");
        edit(p.id);
      })
      .catch((err) => notifCenter.error(err));
  }

  function remove(id: string) {
    ApiRoutes.delete_order({ id })
      .then(() => {
        notifCenter.success("Order deleted");
        pull($page);
      })
      .catch((err) => notifCenter.error(err));
  }

  $: pull($page);
  $: document.title = "Orders";
</script>

<LoadingShow {status}>
  <header>
    <h1>Orders</h1>
    <button
      on:click={() => {
        fields = default_fields();
        dialog.showModal();
      }}
    >
      New
    </button>
  </header>

  {#if orders.data.length}
    <Table>
      <thead>
        <tr>
          <th>Full Name</th>
          <th>Phone</th>
          <th>Email</th>
          <th>Province</th>
          <th>Delivery</th>
          <th>Status</th>
          <th>Options</th>
        </tr>
      </thead>
      <tbody>
        {#each orders.data as order}
          <tr>
            <td>{order.full_name}</td>
            <td>{order.phone}</td>
            <td>{order.email}</td>
            <td>{order.province}</td>
            <td>{order.delivery}</td>
            <td>{order.status}</td>
            <td>
              <button on:click={() => edit(order.id)}>Details</button>
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
      <h2>Create Order</h2>
    </header>
    <form
      on:submit={(ev) => {
        ev.preventDefault();
        create();
      }}
    >
      <fieldset>
        <label>
          Full Name
          <input type="text" bind:value={fields.full_name} required />
        </label>
      </fieldset>
      <fieldset>
        <label>
          Phone
          <input type="tel" bind:value={fields.phone} required />
        </label>
      </fieldset>
      <fieldset>
        <label>
          Email
          <input type="email" bind:value={fields.email} required />
        </label>
      </fieldset>
      <fieldset>
        <label>
          Province
          <input type="text" bind:value={fields.province} required />
        </label>
      </fieldset>
      <fieldset>
        <label>
          Address
          <input type="text" bind:value={fields.address} required />
        </label>
      </fieldset>
      <fieldset>
        <label>
          Delivery
          <select bind:value={fields.delivery}>
            <option>STOP_DESK</option>
            <option>DOMICIL</option>
            <option>OTHER</option>
          </select>
        </label>
      </fieldset>
      <fieldset>
        <label>
          Note
          <input type="text" bind:value={fields.note} required />
        </label>
      </fieldset>
      <button type="button" on:click={() => dialog.close()}>Close</button>
      <button type="submit">Submit</button>
    </form>
  </Dialog>
</LoadingShow>
