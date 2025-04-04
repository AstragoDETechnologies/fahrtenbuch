<template>
    <div class="navbar bg-base-100 shadow-sm">
        <div class="flex-1">
            <NuxtLink to="/" class="btn btn-ghost text-xl">Fahrtenbuch</NuxtLink>
        </div>
        <div class="flex-none">
            <ul class="menu menu-horizontal px-1">
                <li>
                    <NuxtLink to="/dashboard">Dashboard</NuxtLink>
                </li>
                <li>
                    <!-- <details>
                        <summary>Parent</summary>
                        <ul class="bg-base-100 rounded-t-none p-2">
                            <li><a>Link 1</a></li>
                            <li><a>Link 2</a></li>
                        </ul>
                    </details> -->
                </li>
            </ul>
        </div>
    </div>
</template>

<script lang="ts" setup>

</script>

<style></style>