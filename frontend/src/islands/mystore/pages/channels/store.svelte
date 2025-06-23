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
  import { deepEqual, formatDate, isNone } from "../../lib/utils";
  import type { LoadingStatus } from "../../components/LoadingShow.svelte";

  const { replace } = useNavigate();
  const { location } = useRoute();

  let oid: string;
  let status: LoadingStatus = undefined;
  let store: {};

  let fields: {
    edit_basic: boolean;
    edit_theme: boolean;
  };

  function pull() {}

  async function update() {
    try {
    } catch (err) {
      notifCenter.error(err);
    }
  }

  function remove() {}

  function updateFields() {
    fields = {
      edit_basic: false,
      edit_theme: false,
    };
  }

  $: {
    const urlId = location?.params.oid;
    if (urlId) {
      oid = urlId as string;
      pull();
    } else {
      replace({ path: AppRoutes.PRODUCTS.path });
    }
  }
  $: document.title = `Edit Store ${store?.full_name}`;
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
        <div>
          <h2>{store.name}</h2>
          <h3>{oid}</h3>
        </div>
        <Row>
          <button type="reset" on:click={() => remove()}> Delete </button>
          <button type="submit"> Save </button>
        </Row>
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
                Name
                <input
                  readonly={!fields.edit_basic}
                  type="text"
                  bind:value={fields.name}
                  required
                />
              </label>
            </fieldset>
          </Card>
        </Content>
        <Panel></Panel>
      </section>
    </Editor>
  </form>
</LoadingShow>
