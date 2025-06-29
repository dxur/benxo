<script lang="ts">
  import Table from "../../components/Table.svelte";
  import TablePagination from "../../components/TablePagination.svelte";
  import LoadingShow from "../../components/LoadingShow.svelte";
  import Dialog from "../../components/Dialog.svelte";
  import * as ApiRoutes from "@bindings/ApiRoutes";
  import { writable } from "svelte/store";
  import { useNavigate } from "@dvcol/svelte-simple-router";
  import { AppRoutes } from "../../routes";
  import { notifCenter } from "@/stores/notifications";
  import { _ } from "svelte-i18n";
  import type { Page } from "@bindings/Page";
  import type { OrderPublic } from "@bindings/OrderPublic";
  import type { DeliveryType } from "@bindings/DeliveryType";
  import type { LoadingStatus } from "../../components/LoadingShow.svelte";

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
        status = err || $_("common.errors.generic");
      });
  }

  function edit(id: string) {
    push({
      path: AppRoutes.ORDER.path,
      params: { oid: id },
    });
  }

  function create() {
    ApiRoutes.create_order(fields)
      .then((p) => {
        notifCenter.success($_("common.notifications.created"));
        edit(p.id);
      })
      .catch((err) => notifCenter.error(err));
  }

  function remove(id: string) {
    ApiRoutes.delete_order({ id })
      .then(() => {
        notifCenter.success($_("common.notifications.deleted"));
        pull($page);
      })
      .catch((err) => notifCenter.error(err));
  }

  $: pull($page);
  $: document.title = $_("routes.orders.title");
</script>

<LoadingShow {status}>
  <header>
    <h1>{$_("routes.orders.title")}</h1>
    <button
      on:click={() => {
        fields = default_fields();
        dialog.showModal();
      }}
    >
      {$_("common.actions.new")}
    </button>
  </header>

  {#if orders.data.length}
    <Table>
      <thead>
        <tr>
          <th>{$_("common.tableHeaders.fullName")}</th>
          <th>{$_("common.tableHeaders.phone")}</th>
          <th>{$_("common.tableHeaders.email")}</th>
          <th>{$_("common.tableHeaders.province")}</th>
          <th>{$_("common.tableHeaders.delivery")}</th>
          <th>{$_("common.tableHeaders.status")}</th>
          <th>{$_("common.tableHeaders.options")}</th>
        </tr>
      </thead>
      <tbody>
        {#each orders.data as order}
          <tr>
            <td>{order.full_name}</td>
            <td>{order.phone}</td>
            <td>{order.email}</td>
            <td>{order.province}</td>
            <td>{$_(`common.delivery.${order.delivery.toLowerCase()}`)}</td>
            <td>{$_(`common.status.${order.status.toLowerCase()}`)}</td>
            <td>
              <button on:click={() => edit(order.id)}>
                {$_("common.actions.details")}
              </button>
            </td>
          </tr>
        {/each}
      </tbody>
    </Table>
    <TablePagination {page} {total} />
  {:else}
    <span>{$_("common.emptyState")}</span>
  {/if}

  <Dialog bind:dialog>
    <header>
      <h2>{$_("pages.orders.dialog.title")}</h2>
    </header>
    <form
      on:submit={(ev) => {
        ev.preventDefault();
        create();
      }}
    >
      <fieldset>
        <label>
          {$_("common.form.fullName")}
          <input type="text" bind:value={fields.full_name} required />
        </label>
      </fieldset>
      <fieldset>
        <label>
          {$_("common.form.phone")}
          <input type="tel" bind:value={fields.phone} required />
        </label>
      </fieldset>
      <fieldset>
        <label>
          {$_("common.form.email")}
          <input type="email" bind:value={fields.email} required />
        </label>
      </fieldset>
      <fieldset>
        <label>
          {$_("common.form.province")}
          <input type="text" bind:value={fields.province} required />
        </label>
      </fieldset>
      <fieldset>
        <label>
          {$_("common.form.address")}
          <input type="text" bind:value={fields.address} required />
        </label>
      </fieldset>
      <fieldset>
        <label>
          {$_("common.form.delivery")}
          <select bind:value={fields.delivery}>
            <option value="STOP_DESK">{$_("common.delivery.stop_desk")}</option>
            <option value="DOMICIL">{$_("common.delivery.domicil")}</option>
            <option value="OTHER">{$_("common.delivery.other")}</option>
          </select>
        </label>
      </fieldset>
      <fieldset>
        <label>
          {$_("common.form.note")}
          <input type="text" bind:value={fields.note} required />
        </label>
      </fieldset>
      <button type="submit">{$_("common.actions.submit")}</button>
      <button type="button" on:click={() => dialog.close()}>
        {$_("common.actions.close")}
      </button>
    </form>
  </Dialog>
</LoadingShow>
