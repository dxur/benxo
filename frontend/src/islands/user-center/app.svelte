<script lang="ts">
  import * as Card from "$lib/components/ui/card/index.js";
  import { Label } from "$lib/components/ui/label/index.js";
  import { Input } from "$lib/components/ui/input/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import { Textarea } from "$lib/components/ui/textarea/index.js";
  import * as Select from "$lib/components/ui/select/index.js";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import * as Tabs from "$lib/components/ui/tabs/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import {
    PlusIcon,
    BuildingIcon,
    MailIcon,
    UserIcon,
    SettingsIcon,
    TrashIcon,
    SendIcon,
    CheckIcon,
    XIcon,
    ExternalLinkIcon,
    LogOutIcon,
  } from "@lucide/svelte";

  import {
    list,
    create,
    switch_business,
    current,
    create_invitation,
    accept_invitation,
    resend_invitation,
    get_pending_invitations,
    update_member,
    remove_member,
    update_settings,
  } from "@bindings/BusinessRoutes";
  import { logout } from "@bindings/UserRoutes";
  import { me } from "@bindings/UserRoutes";
  import type { BusinessDto } from "@bindings/BusinessDto";
  import type { InvitationDto } from "@bindings/InvitationDto";
  import type { BusinessMemberDto } from "@bindings/BusinessMemberDto";
  import type { PendingInvitationsResponse } from "@bindings/PendingInvitationsResponse";
  import { navigate } from "astro:transitions/client";
  // import { ModeWatcher } from "mode-watcher";

  // State
  let businesses: BusinessDto[] = $state([]);
  let currentBusiness: BusinessDto | null = $state(null);
  let pendingInvitations: [BusinessDto, InvitationDto][] = $state([]);
  let members: BusinessMemberDto[] = $state([]);
  let loading = $state(false);
  let error = $state<string | null>(null);

  // Dialog states
  let createBusinessOpen = $state(false);
  let inviteUserOpen = $state(false);
  let settingsOpen = $state(false);

  // Form states
  let businessForm = $state({
    name: "",
    description: "",
  });

  let inviteForm = $state({
    email: "",
    role: "member" as const,
    permissions: null as any,
  });

  let settingsForm = $state({
    name: "",
    description: "",
    settings: {},
  });

  // Load data on mount
  $effect(() => {
    loadUserData();
  });

  async function loadUserData() {
    loading = true;
    error = null;
    try {
      await Promise.all([
        loadBusinesses(),
        loadCurrentBusiness(),
        loadPendingInvitations(),
      ]);
    } catch (err) {
      error = (err as string) || "Failed to load data";
    } finally {
      loading = false;
    }
  }

  async function loadBusinesses() {
    const response = await list();
    businesses = response.businesses;
  }

  async function loadCurrentBusiness() {
    try {
      currentBusiness = await current();
      if (currentBusiness) {
        settingsForm.name = currentBusiness.name;
        settingsForm.description = currentBusiness.description || "";
      }
    } catch {
      // No current business selected
      currentBusiness = null;
    }
  }

  async function loadPendingInvitations() {
    const response = await get_pending_invitations();
    pendingInvitations = response.invitations;
  }

  async function createBusiness(e: Event) {
    e.preventDefault();
    loading = true;
    try {
      await create({
        name: businessForm.name,
        description: businessForm.description || null,
      });
      businessForm.name = "";
      businessForm.description = "";
      createBusinessOpen = false;
      await loadBusinesses();
    } catch (err) {
      error = (err as string) || "Failed to create business";
    } finally {
      loading = false;
    }
  }

  async function switchBusiness(businessId: string) {
    loading = true;
    try {
      await switch_business({ business_id: businessId });
      // navigate("/business-center/");
      await loadCurrentBusiness();
    } catch (err) {
      error = (err as string) || "Failed to switch business";
    } finally {
      loading = false;
    }
  }

  async function sendInvitation(e: Event) {
    e.preventDefault();
    loading = true;
    try {
      await create_invitation({
        email: inviteForm.email,
        role: inviteForm.role,
        permissions: inviteForm.permissions,
      });
      inviteForm.email = "";
      inviteForm.role = "member";
      inviteForm.permissions = null;
      inviteUserOpen = false;
    } catch (err) {
      error = (err as string) || "Failed to send invitation";
    } finally {
      loading = false;
    }
  }

  async function acceptInvite(token: string) {
    loading = true;
    try {
      await accept_invitation({ token });
      await loadPendingInvitations();
      await loadBusinesses();
    } catch (err) {
      error = (err as string) || "Failed to accept invitation";
    } finally {
      loading = false;
    }
  }

  async function updateBusinessSettings(e: Event) {
    e.preventDefault();
    if (!currentBusiness) return;

    loading = true;
    try {
      // await update_settings({
      //   name: settingsForm.name,
      //   description: settingsForm.description,
      //   settings: settingsForm.settings,
      // });
      alert("Settings update is not implemented yet.");
      settingsOpen = false;
      await loadCurrentBusiness();
    } catch (err) {
      error = (err as string) || "Failed to update settings";
    } finally {
      loading = false;
    }
  }

  async function handleLogout() {
    loading = true;
    try {
      await logout();
      navigate("/");
    } catch (err) {
      error = (err as string) || "Failed to logout";
    } finally {
      loading = false;
    }
  }
</script>

<!-- <ModeWatcher /> -->

<div
  class="flex min-h-svh flex-col items-center justify-center p-6 md:p-10 bg-gradient-to-b from-gray-400/50 via-gray-800/70 to-gray-900/80 z-0"
>
  <div class="w-full max-w-sm md:max-w-3xl">
    <div class="container mx-auto p-6 space-y-6">
      {#if error}
        <div
          class="rounded-md bg-red-50 p-3 text-sm text-red-600 border border-red-200"
        >
          {error}
        </div>
      {/if}

      <!-- Header -->
      <div class="flex justify-between items-center">
        <div class="flex flex-col gap-2">
          <h1 class="text-3xl font-bold">User Center</h1>
          <p class="text-muted-foreground">
            Manage your businesses, invitations, and account settings
          </p>
        </div>
        <div>
          <Button variant="destructive" onclick={handleLogout}
            ><LogOutIcon /></Button
          >
        </div>
      </div>

      <Tabs.Root value="businesses" class="w-full">
        <Tabs.List class="grid w-full grid-cols-3">
          <Tabs.Trigger value="businesses">My Businesses</Tabs.Trigger>
          <Tabs.Trigger value="invitations">Invitations</Tabs.Trigger>
          <Tabs.Trigger value="settings">Settings</Tabs.Trigger>
        </Tabs.List>
        <Card.Root>
          <Card.Content>
            <!-- Businesses Tab -->
            <Tabs.Content value="businesses" class="space-y-4">
              <div class="flex items-center justify-between">
                <h2 class="text-xl font-semibold">Your Businesses</h2>
                <Button onclick={() => (createBusinessOpen = true)}>
                  <PlusIcon class="w-4 h-4" />
                  Create Business
                </Button>
              </div>

              {#if currentBusiness}
                <Card.Root>
                  <Card.Header>
                    <div
                      class="flex flex-col md:flex-row gap-4 md:gap-1 items-center justify-between"
                    >
                      <div class="flex items-center gap-2">
                        <BuildingIcon class="w-5 h-5 text-green-600" />
                        <div class="space-y-1">
                          <Card.Title class="text-green-800"
                            >Current Business</Card.Title
                          >
                          <Card.Description class="text-green-700"
                            >{currentBusiness.name}</Card.Description
                          >
                        </div>
                      </div>
                      <div
                        class="flex flex-col md:flex-row gap-2 w-full justify-center md:w-auto"
                      >
                        <Button
                          variant="outline"
                          size="sm"
                          onclick={() => (settingsOpen = true)}
                        >
                          <SettingsIcon class="w-4 h-4 mr-1" />
                          Settings
                        </Button>
                        <Button
                          variant="outline"
                          size="sm"
                          onclick={() => navigate("/business-center/")}
                        >
                          <ExternalLinkIcon class="w-4 h-4 mr-1" />
                          Open
                        </Button>
                      </div>
                    </div>
                  </Card.Header>
                  {#if currentBusiness.description}
                    <Card.Content>
                      <p class="text-green-800">
                        {currentBusiness.description}
                      </p>
                    </Card.Content>
                  {/if}
                </Card.Root>
              {/if}

              <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
                {#each businesses as business}
                  {#if !currentBusiness || business.id !== currentBusiness.id}
                    <Card.Root
                      class="hover:shadow-md transition-shadow cursor-pointer"
                    >
                      <Card.Header>
                        <div class="flex items-center justify-between">
                          <div class="flex items-center gap-2">
                            <BuildingIcon class="w-4 h-4" />
                            <Card.Title class="text-sm"
                              >{business.name}</Card.Title
                            >
                          </div>
                          <Badge variant="secondary">{business.plan_type}</Badge
                          >
                        </div>
                        {#if business.description}
                          <Card.Description
                            >{business.description}</Card.Description
                          >
                        {/if}
                      </Card.Header>
                      <Card.Content>
                        <Button
                          size="sm"
                          class="w-full"
                          disabled={loading}
                          onclick={() => switchBusiness(business.id)}
                        >
                          Switch to Business
                        </Button>
                      </Card.Content>
                    </Card.Root>
                  {/if}
                {/each}
              </div>

              {#if businesses.length === 0}
                <Card.Root>
                  <Card.Content
                    class="flex flex-col items-center justify-center py-12"
                  >
                    <BuildingIcon
                      class="w-12 h-12 text-muted-foreground mb-4"
                    />
                    <h3 class="text-lg font-medium mb-2">No businesses yet</h3>
                    <p class="text-muted-foreground mb-4 text-center">
                      Create your first business to get started with managing
                      your operations.
                    </p>
                    <Button onclick={() => (createBusinessOpen = true)}>
                      <PlusIcon class="w-4 h-4 mr-2" />
                      Create Your First Business
                    </Button>
                  </Card.Content>
                </Card.Root>
              {/if}
            </Tabs.Content>

            <!-- Invitations Tab -->
            <Tabs.Content value="invitations" class="space-y-4">
              <div class="flex items-center justify-between">
                <h2 class="text-xl font-semibold">Pending Invitations</h2>
                {#if currentBusiness}
                  <Button onclick={() => (inviteUserOpen = true)}>
                    <MailIcon class="w-4 h-4 mr-2" />
                    Send Invitation
                  </Button>
                {/if}
              </div>

              {#if pendingInvitations.length > 0}
                <div class="space-y-3">
                  {#each pendingInvitations as [_, invitation]}
                    <Card.Root>
                      <Card.Content
                        class="flex items-center justify-between p-4"
                      >
                        <div class="flex items-center gap-3">
                          <MailIcon class="w-4 h-4 text-blue-500" />
                          <div>
                            <p class="font-medium">{invitation.email}</p>
                            <div
                              class="flex items-center gap-2 text-sm text-muted-foreground"
                            >
                              <Badge variant="outline">{invitation.role}</Badge>
                              <span>•</span>
                              <span
                                >Expires: {new Date(
                                  invitation.expires_at,
                                ).toLocaleDateString()}</span
                              >
                            </div>
                          </div>
                        </div>
                        <div class="flex gap-2">
                          <Button
                            size="sm"
                            onclick={() => acceptInvite(invitation.token)}
                            disabled={loading}
                          >
                            <CheckIcon class="w-4 h-4 mr-1" />
                            Accept
                          </Button>
                          <Button size="sm" variant="outline">
                            <XIcon class="w-4 h-4" />
                          </Button>
                        </div>
                      </Card.Content>
                    </Card.Root>
                  {/each}
                </div>
              {:else}
                <Card.Root>
                  <Card.Content
                    class="flex flex-col items-center justify-center py-12"
                  >
                    <MailIcon class="w-12 h-12 text-muted-foreground mb-4" />
                    <h3 class="text-lg font-medium mb-2">
                      No pending invitations
                    </h3>
                    <p class="text-muted-foreground text-center">
                      You don't have any pending business invitations at the
                      moment.
                    </p>
                  </Card.Content>
                </Card.Root>
              {/if}
            </Tabs.Content>

            <!-- Settings Tab -->
            <Tabs.Content value="settings" class="space-y-4">
              <h2 class="text-xl font-semibold">Account Settings</h2>

              <Card.Root>
                <Card.Header>
                  <Card.Title>Profile Information</Card.Title>
                  <Card.Description>
                    Update your personal account information
                  </Card.Description>
                </Card.Header>
                <Card.Content class="space-y-4">
                  <div
                    class="grid grid-cols-1 md:grid-cols-2 gap-4 [&>*]:space-y-2"
                  >
                    <div>
                      <Label for="first-name">First Name</Label>
                      <Input id="first-name" placeholder="John" />
                    </div>
                    <div>
                      <Label for="last-name">Last Name</Label>
                      <Input id="last-name" placeholder="Doe" />
                    </div>
                  </div>
                  <div class="space-y-2">
                    <Label for="email">Email</Label>
                    <Input
                      id="email"
                      type="email"
                      placeholder="john@example.com"
                    />
                  </div>
                  <Button>Save Changes</Button>
                </Card.Content>
              </Card.Root>
            </Tabs.Content>
          </Card.Content>
        </Card.Root>
      </Tabs.Root>
    </div>
  </div>
</div>

<!-- Create Business Dialog -->
<Dialog.Root bind:open={createBusinessOpen}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Create New Business</Dialog.Title>
      <Dialog.Description>
        Set up a new business to start managing your operations.
      </Dialog.Description>
    </Dialog.Header>

    <form onsubmit={createBusiness} class="space-y-4 [&>*]:space-y-2">
      <div>
        <Label for="business-name">Business Name</Label>
        <Input
          id="business-name"
          bind:value={businessForm.name}
          placeholder="Acme Corp"
          required
        />
      </div>

      <div>
        <Label for="business-description">Description (Optional)</Label>
        <Textarea
          id="business-description"
          bind:value={businessForm.description}
          placeholder="Brief description of your business..."
          rows={3}
        />
      </div>

      <div class="flex justify-end gap-2">
        <Button
          type="button"
          variant="outline"
          onclick={() => (createBusinessOpen = false)}
        >
          Cancel
        </Button>
        <Button type="submit" disabled={loading}>
          {loading ? "Creating..." : "Create Business"}
        </Button>
      </div>
    </form>
  </Dialog.Content>
</Dialog.Root>

<!-- Send Invitation Dialog -->
<Dialog.Root bind:open={inviteUserOpen}>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Invite User</Dialog.Title>
      <Dialog.Description>
        Send an invitation to join your business.
      </Dialog.Description>
    </Dialog.Header>

    <form onsubmit={sendInvitation} class="space-y-4 [&>*]:space-y-2">
      <div>
        <Label for="invite-email">Email Address</Label>
        <Input
          id="invite-email"
          type="email"
          bind:value={inviteForm.email}
          placeholder="user@example.com"
          required
        />
      </div>

      <div>
        <Label for="invite-role">Role</Label>
        <Select.Root type="single" bind:value={inviteForm.role}>
          <Select.Trigger>Select a role...</Select.Trigger>
          <Select.Content>
            <Select.Item value="member">Member</Select.Item>
            <Select.Item value="manager">Manager</Select.Item>
            <Select.Item value="admin">Admin</Select.Item>
          </Select.Content>
        </Select.Root>
      </div>

      <div class="flex justify-end gap-2">
        <Button
          type="button"
          variant="outline"
          onclick={() => (inviteUserOpen = false)}
        >
          Cancel
        </Button>
        <Button type="submit" disabled={loading}>
          <SendIcon class="w-4 h-4 mr-2" />
          {loading ? "Sending..." : "Send Invitation"}
        </Button>
      </div>
    </form>
  </Dialog.Content>
</Dialog.Root>

<!-- Business Settings Dialog -->
<Dialog.Root bind:open={settingsOpen}>
  <Dialog.Content class="sm:max-w-[500px]">
    <Dialog.Header>
      <Dialog.Title>Business Settings</Dialog.Title>
      <Dialog.Description>
        Update your business information and settings.
      </Dialog.Description>
    </Dialog.Header>

    <form onsubmit={updateBusinessSettings} class="space-y-4 [&>*]:space-y-2">
      <div>
        <Label for="settings-name">Business Name</Label>
        <Input
          id="settings-name"
          bind:value={settingsForm.name}
          placeholder="Business name"
          required
        />
      </div>

      <div>
        <Label for="settings-description">Description</Label>
        <Textarea
          id="settings-description"
          bind:value={settingsForm.description}
          placeholder="Business description..."
          rows={3}
        />
      </div>

      <div class="flex justify-end gap-2">
        <Button
          type="button"
          variant="outline"
          onclick={() => (settingsOpen = false)}
        >
          Cancel
        </Button>
        <Button type="submit" disabled={loading}>
          {loading ? "Saving..." : "Save Changes"}
        </Button>
      </div>
    </form>
  </Dialog.Content>
</Dialog.Root>
