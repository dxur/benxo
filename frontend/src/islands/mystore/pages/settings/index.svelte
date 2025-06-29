<script lang="ts">
  import LoadingShow from "../../components/LoadingShow.svelte";
  import Editor from "../../components/Editor.svelte";
  import Row from "../../components/Row.svelte";
  import Content from "../../components/Content.svelte";
  import Panel from "../../components/Panel.svelte";
  import Card from "../../components/Card.svelte";
  import Badge from "../../components/Badge.svelte";
  import Table from "../../components/Table.svelte";
  import Timeline from "../../components/Timeline.svelte";
  import Dialog from "../../components/Dialog.svelte";
  import { notifCenter } from "@/stores/notifications";
  import * as ApiRoutes from "@bindings/ApiRoutes";
  import { AppRoutes } from "../../routes";
  import { useRoute } from "@dvcol/svelte-simple-router/router";
  import { useNavigate } from "@dvcol/svelte-simple-router/router";
  import { deepEqual, isNone } from "../../lib/utils";
  import type { LoadingStatus } from "../../components/LoadingShow.svelte";
  import type { SettingsPublic } from "@bindings/SettingsPublic";

  interface CartItemEntry {
    sku: string;
    quantity: number;
    price: number;
  }

  let status: LoadingStatus = undefined;
  let settings: SettingsPublic;

  let fields: {
    edit_basic: boolean;
    name: string;
    email: string;
    phone: string;
  };

  function pull() {
    ApiRoutes.get_settings()
      .then((data) => {
        settings = data;
        updateFields();
        status = null;
      })
      .catch((err) => {
        notifCenter.error(err);
        status = err || "An error occurred";
      });
  }

  function update() {
    try {
      const body = tryUpdate();
      if (isNone(body)) {
        notifCenter.warning("No changes made");
        return;
      }
      // ApiRoutes.update_settings({ body }).then((data) => {
      //   notifCenter.success("Order updated");
      //   settings = data;
      //   updateFields();
      // });
    } catch (err) {
      notifCenter.error(err);
    }
  }

  function updateFields() {
    fields = {
      edit_basic: false,
      name: settings.name,
      email: settings.email,
      phone: settings.phone,
    };
  }

  function tryUpdate() {}

  $: pull();
  $: document.title = `Settings`;
</script>

<LoadingShow {status}>
  <form
    on:submit={(ev) => {
      ev.preventDefault();
      update();
    }}
  >
    <Editor>
      <header>
        <h1>Settings</h1>
        <button type="submit"> Save </button>
      </header>
      <section>
        <Content>
          <Card>
            <Row>
              <h3>Basic</h3>
              <button
                type="button"
                on:click={() => {
                  fields.edit_basic = !fields.edit_basic;
                }}
              >
                {#if fields.edit_basic}
                  Done{:else}Edit
                {/if}
              </button>
            </Row>
            <fieldset>
              <label>
                Store Name
                <input
                  readonly={!fields.edit_basic}
                  type="text"
                  placeholder="Awesome Store"
                  bind:value={fields.name}
                  required
                />
              </label>
            </fieldset>
            <fieldset>
              <label>
                Phone
                <input
                  readonly={!fields.edit_basic}
                  type="tel"
                  bind:value={fields.phone}
                  required
                />
              </label>
            </fieldset>
            <fieldset>
              <label>
                Email
                <input
                  readonly={!fields.edit_basic}
                  type="text"
                  placeholder="emain@example.com"
                  bind:value={fields.email}
                  required
                />
              </label>
            </fieldset>
          </Card>
        </Content>
        <Panel>
          <Card>
            <Row>
              <h3>Something</h3>
              <button type="button"> Edit </button>
            </Row>
            <fieldset>
              <label>
                <input
                  readonly={true}
                  type="text"
                  placeholder="Something"
                  pattern="^[a-zA-Z0-9]+(?:-[a-zA-Z0-9]+)*$"
                  required
                />
              </label>
            </fieldset>
          </Card>
        </Panel>
      </section>
    </Editor>
  </form>
</LoadingShow>
