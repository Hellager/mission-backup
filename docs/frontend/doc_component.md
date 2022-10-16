# Component

## Lock

```vue
<script lang="ts" setup>
import Lock from '../components/Lock.vue'
</script>

<template>
  ...
  <Lock :tray="['lock', 'edit', 'add', 'setting', 'statistic']" />
  ...
</template>
```

- props
  - tray

锁屏 + 托盘，通过给定的数组动态生成托盘



## LockCover

```vue
<script lang="ts" setup>
import { ref } from 'vue'
import LockCover from '../components/LockCover.vue'

const locked = ref(false)

const handleValidate = (res: boolean) => {
  if (res)
    locked.value = false

}
</script>

<template>
  ...
  <LockCover :is-locked="locked" @validate="handleValidate" />
  ...
</template>
```

- props
  - isLocked

- functions
  - validate

通过修改 z-index 实现顶层 div 锁屏，isLocked 参数修改状态，validate 校验

## NavigateSideBar

```vue
<script lang="ts" setup>
import NavigateSideBar from '../components/NavigateSideBar.vue'
</script>

<template>
  ...
  <NavigateSideBar position="right" to-path="/" />
  ...
</template>
```

- props
  - position
  - toPath

导航侧边栏，通过 position 指定位置，toPath 指定路由跳转界面

## NavigateTray

```vue
<script lang="ts" setup>
import { ref } from 'vue'
import NavigateTray from '../components/NavigateTray.vue'

const tray = ref(['lock', 'edit', 'add', 'setting', 'statistic'])

const pageLock = ref(false)

const handleValidate = (res: boolean) => {
  if (res)
    pageLock.value = false

}

const togglePageLock = () => {
  if (!pageLock.value)
    pageLock.value = true

}
</script>

<template>
  ...
  <LockCover :is-locked="pageLock" @validate="handleValidate" />
  <NavigateTray :fns="tray" @toggleLock="togglePageLock" />
  ...
</template>
```

- props
  - fns
- functions
  - toggleLock

固定于页面底部的托盘组件，接受字符串数组动态生成托盘，与 LockCover 组件配合实现锁屏

## PageHeader

```vue
<script lang="ts" setup>
import { ref } from 'vue'
import PageHeader from '../components/PageHeader.vue'

const pageTitle = ref('This is the page header')
</script>

<template>
  ...
  <PageHeader :title="pageTitle" to="/" />
  ...
</template>
```

- props
  - title
  - to

页头组件，通过 title 设置标题，to 决定页头组件点击时的路由跳转

## TitleBar

```vue
<script lang="ts" setup>
import TitleBar from './components/TitleBar.vue'
</script>

<template>
  ...
  <TitleBar />
  ...
</template>
```

软件顶部状态栏组件，包括图标、标题及窗口操作按钮