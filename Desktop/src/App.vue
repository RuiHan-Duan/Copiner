<script setup lang="ts">
const stages = ['澄清', '规划', '执行', '总结归档']
</script>

<template>
  <main class="shell">
    <aside>
      <div class="brand"><span class="coin">C</span><div><b>Copiner</b><small>Local-first coworker</small></div></div>
      <button class="new-task">＋ 新任务</button>
      <nav><span class="active">当前会话</span><span>任务归档</span><span>Skills</span><span>记忆体</span></nav>
      <p class="privacy">● 本地模式 · 未连接模型</p>
    </aside>
    <section class="workspace">
      <header><div class="progress"><span v-for="(stage, index) in stages" :key="stage" :class="{ on: index === 0 }">{{ index + 1 }} {{ stage }}</span></div><button>设置</button></header>
      <div class="empty">
        <div class="pet">◇</div>
        <h1>今天想一起完成什么？</h1>
        <p>我会先澄清目标、列出 Todo 和 Plan；得到你的确认后才执行。</p>
        <div class="suggestions"><button>检索 BIOS / BMC 文档</button><button>整理邮件与待办</button><button>分析问题单</button></div>
      </div>
      <footer><textarea aria-label="任务输入" placeholder="描述任务；涉及敏感数据时请先标注……"></textarea><button class="send" disabled>↑</button><small>尚未配置 Provider，当前仅展示交互骨架</small></footer>
    </section>
    <aside class="panel"><h2>Todo</h2><p>需求澄清后将在这里生成任务清单。</p><h2>Plan</h2><p>执行方案会停在确认边界，未经批准不会运行工具。</p><div class="guard">安全护栏已启用<br><small>外部副作用需要人工确认</small></div></aside>
  </main>
</template>

