<template>
  <div class="h-screen flex flex-col bg-gray-50 dark:bg-gray-900">
    <AppHeader :is-running="isRunning"
               :env-installed="envInfo.installed"
               :supported-languages="supportedLanguages"
               :current-language="currentLanguage"
               :current-layout="layoutMode"
               :sidebar-visible="sidebarVisible"
               @toggle-sidebar="toggleSidebar"
               @run-code="handleRunCode"
               @stop-code="stopCode"
               @language-change="onLanguageChange"
               @layout-change="handleLayoutChange"
               @open-file="handleOpenFileClick"
               @save-file="handleSave"
               @show-history="showHistory = true"
               @show-ai="handleShowAi"
               @show-git="openGit"
               @show-settings="showSettings = true"
               @load-example="loadExample">
    </AppHeader>

    <!-- 运行输入：参数 + stdin（任何布局/运行前都可填）-->
    <div class="bg-gray-50 dark:bg-gray-900 border-b border-gray-200 dark:border-gray-700 flex-shrink-0">
      <button class="w-full flex items-center px-4 py-1 text-xs text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="showRunInput = !showRunInput">
        <ChevronRight class="w-3 h-3 mr-1 transition-transform" :class="{ 'rotate-90': showRunInput }"/>
        {{ t('app.runInputToggle') }}
        <span v-if="!showRunInput && (runArgs || runStdin || runEnv)" class="ml-2 text-blue-500">●</span>
      </button>
      <div v-if="showRunInput" class="px-4 pb-2 space-y-2">
        <div class="flex items-start space-x-3">
          <div class="flex flex-col w-56 flex-shrink-0">
            <label class="text-[11px] text-gray-400 mb-0.5">{{ t('app.runArgs') }}</label>
            <input v-model="runArgs" class="text-xs border border-gray-300 rounded px-2 py-1 focus:outline-none focus:border-blue-400" :placeholder="t('app.runArgsPlaceholder')"/>
          </div>
          <div class="flex flex-col flex-1 min-w-0">
            <label class="text-[11px] text-gray-400 mb-0.5">{{ t('app.stdin') }}</label>
            <textarea v-model="runStdin" rows="2" class="w-full text-xs border border-gray-300 rounded px-2 py-1 font-mono resize-none focus:outline-none focus:border-blue-400" :placeholder="t('app.stdinPlaceholder')"></textarea>
          </div>
        </div>
        <div class="flex flex-col">
          <label class="text-[11px] text-gray-400 mb-0.5">{{ t('app.envVars') }}</label>
          <input v-model="runEnv" class="text-xs border border-gray-300 rounded px-2 py-1 font-mono focus:outline-none focus:border-blue-400" :placeholder="t('app.envVarsPlaceholder')"/>
        </div>
        <label class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400 cursor-pointer select-none">
          <input v-model="watchMode" type="checkbox" class="cursor-pointer"/>
          {{ t('app.watchMode') }}
        </label>
      </div>
    </div>

    <div class="flex-1 min-h-0 overflow-hidden flex">
      <!-- 左侧文件树侧栏 -->
      <template v-if="sidebarVisible">
        <Sidebar :root-dir="rootDir"
                 :extra-roots="extraRoots"
                 :reveal-request="revealRequest"
                 :active-path="currentFilePath"
                 :recent-folders="recentFolders"
                 :git-status="gitStatus"
                 :git-repo="gitRepo"
                 class="flex-shrink-0"
                 :style="{ width: `${sidebarWidth}px` }"
                 @open-folder="openFolder"
                 @add-folder="addWorkspaceFolder"
                 @remove-root="removeWorkspaceFolder"
                 @open-recent="openFolderPath"
                 @open-file="smartOpen"
                 @search-in="openSearchInFolder"
                 @renamed="(from, to) => updateTabPath(from, to)"
                 @deleted="(p) => detachTabPath(p)"
                 @git-refresh="refreshGitStatus"/>
        <!-- 拖拽改变侧栏宽度 -->
        <div class="w-1 bg-gray-200 dark:bg-gray-700 hover:bg-blue-500 cursor-col-resize transition-colors flex-shrink-0"
             @mousedown="startSidebarResize"></div>
      </template>

      <div class="flex-1 min-h-0 overflow-hidden">
      <!-- 编辑器代码片段 -->
      <template v-if="showConsole">
        <ResizablePanels :direction="effectiveDirection" :min-primary="minPrimary" :min-secondary="minSecondary">
          <template #primary>
            <div class="h-full flex flex-col overflow-hidden">
              <EditorTabs :tabs="editorTabs" :active-id="activeTabId" @switch="switchTab" @close="handleCloseTab" @new="handleNewTab"
                          @close-others="closeOthers" @close-right="closeToRight" @move="moveTab" @copy-path="handleCopyPath"
                          @copy-relative="handleCopyRelativePath" @reveal-tree="revealInTree" @reveal-finder="revealInFinder" @toggle-pin="togglePin"/>
              <div v-if="!showViewer" class="bg-gray-100 dark:bg-gray-800 px-4 py-2 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between flex-shrink-0">
                <div class="flex items-center space-x-3 min-w-0 flex-1 overflow-x-auto [&::-webkit-scrollbar]:hidden [-ms-overflow-style:none] [scrollbar-width:none]">
                  <img :src="`/icons/${currentLanguage.replace(/\d+$/, '')}.svg`" class="w-5 h-5 flex-shrink-0" :alt="currentLanguage" @error="onIconError"/>
                  <h2 class="text-sm font-medium text-gray-700 dark:text-gray-200 whitespace-nowrap flex-shrink-0">{{ getLanguageDisplayName(currentLanguage) }} {{ t('app.codeEditor') }}</h2>
                  <template v-if="currentFilePath">
                    <span class="text-gray-400 text-xs flex-shrink-0">·</span>
                    <div class="min-w-0 overflow-x-auto [&::-webkit-scrollbar]:hidden [-ms-overflow-style:none] [scrollbar-width:none]">
                      <Breadcrumbs :path="currentFilePath" :root-dir="rootDir" :dirty="isDirty" @reveal="revealInFinder" @open="smartOpen"/>
                    </div>
                  </template>
                  <span v-else-if="currentFileName" class="text-xs text-gray-500 flex items-center whitespace-nowrap flex-shrink-0">
                    · {{ currentFileName }}
                    <span v-if="isDirty" class="ml-1 text-amber-500" :title="t('app.unsaved')">●</span>
                  </span>
                  <SqlSourceSelect v-if="currentLanguage === 'sql'" class="flex-shrink-0"/>
                  <SchemaBrowser v-if="currentLanguage === 'sql'" class="flex-shrink-0" @preview="previewTable" @insert="insertAtCursor"/>
                  <AiSql v-if="currentLanguage === 'sql'" class="flex-shrink-0" @generated="insertAtCursor"/>
                  <ErDiagram v-if="currentLanguage === 'sql'" class="flex-shrink-0"/>
                  <TxnControl v-if="currentLanguage === 'sql'" class="flex-shrink-0" @notice="onTxnNotice"/>
                </div>

                <div class="flex items-center space-x-2 text-xs text-gray-500 whitespace-nowrap flex-shrink-0 pl-3">
                  <span v-if="predicting" class="flex items-center gap-1 text-blue-500">
                    <Sparkles class="w-3 h-3 animate-pulse"/> {{ t('app.aiPredicting') }}
                  </span>
                  <span v-else-if="ghostActive" class="text-blue-500">{{ t('app.ghostHint') }}</span>
                  <span>{{ t('app.cursorPos', { line: cursorInfo.line, col: cursorInfo.col }) }}</span>
                  <span v-if="cursorInfo.selLen">{{ t('app.selected') }} <strong>{{ cursorInfo.selLen }}</strong></span>
                  <span><strong>{{ (code || '').length }}</strong> {{ t('app.chars') }}</span>
                  <span><strong>{{ (code || '').split('\n').length }}</strong> {{ t('app.lines') }}</span>
                  <IndentControl v-if="editorConfig" :config="editorConfig"/>
                </div>
              </div>
              <div class="flex-1 overflow-hidden relative">
                <CodeEditor v-model="code" class="h-full" :language="currentLanguage" :file-path="currentFilePath" :root-dir="rootDir" :editor-config="editorConfig" :key="editorConfigKey" @ready="editorView = $event"/>
                <LargeFileViewer v-if="showViewer && viewerFile"
                                 :file-path="viewerFile.path"
                                 :line-count="viewerFile.lineCount"
                                 :size-bytes="viewerFile.sizeBytes"
                                 @close="closeViewer"/>
                <InlineGenerate v-if="showGenerate" :language="currentLanguage" :selection="generateSelection" @insert="insertGeneratedCode" @close="showGenerate = false"/>
              </div>
            </div>
          </template>

          <template #secondary>
            <!-- 输出 -->
            <div class="h-full min-h-0 flex flex-col overflow-hidden" :class="effectiveDirection === 'vertical' ? 'border-t border-gray-200' : 'border-l border-gray-200'">
              <!-- 仅编辑器模式下提供收起控制台的入口 -->
              <div v-if="layoutMode === 'editor'" class="bg-gray-100 dark:bg-gray-800 px-4 py-2 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between flex-shrink-0">
                <h2 class="text-sm font-medium text-gray-700 dark:text-gray-200">{{ t('console.title') }}</h2>
                <button class="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors" :title="t('app.collapseConsole')" @click="showConsole = false">
                  <X class="w-4 h-4"/>
                </button>
              </div>

              <ConsoleOutput v-if="consoleType === 'console'"
                             class="flex-1 min-h-0"
                             :output="output"
                             :is-running="isRunning"
                             :is-success="isSuccess"
                             :execution-time="lastExecutionTime"
                             @clear="clearOutput">
              </ConsoleOutput>

              <!-- Web输出组件 -->
              <WebOutput v-else-if="consoleType === 'web'"
                         class="flex-1 min-h-0"
                         :web-content="output"
                         :is-running="isRunning"
                         :execution-time="lastExecutionTime"
                         @clear="clearOutput">
              </WebOutput>

              <!-- JSON 视图 -->
              <JsonView v-else-if="consoleType === 'json'"
                        class="flex-1 min-h-0"
                        :output="output"
                        :is-running="isRunning"
                        :execution-time="lastExecutionTime"
                        @clear="clearOutput"/>

              <!-- Markdown 预览 -->
              <MarkdownView v-else-if="consoleType === 'markdown'"
                            class="flex-1 min-h-0"
                            :output="output"
                            :is-running="isRunning"
                            :execution-time="lastExecutionTime"
                            @clear="clearOutput"/>

              <!-- XML 视图 -->
              <XmlView v-else-if="consoleType === 'xml'"
                       class="flex-1 min-h-0"
                       :output="output"
                       :is-running="isRunning"
                       :execution-time="lastExecutionTime"
                       @clear="clearOutput"/>

              <!-- YAML 视图 -->
              <YamlView v-else-if="consoleType === 'yaml'"
                        class="flex-1 min-h-0"
                        :output="output"
                        :is-running="isRunning"
                        :execution-time="lastExecutionTime"
                        @clear="clearOutput"/>

              <!-- SQL 表格 -->
              <SqlTableView v-else-if="consoleType === 'sqltable'"
                            class="flex-1 min-h-0"
                            :output="output"
                            :is-running="isRunning"
                            :execution-time="lastExecutionTime"
                            :paging="sqlPaging"
                            @prev="sqlPrevPage"
                            @next="sqlNextPage"
                            @clear="clearOutput"/>

              <!-- 数据表 / 图表（CSV / TSV） -->
              <DataTableView v-else-if="consoleType === 'table'"
                             class="flex-1 min-h-0"
                             :output="output"
                             :is-running="isRunning"
                             :execution-time="lastExecutionTime"
                             @clear="clearOutput"/>

              <!-- Excel 表 / 图表（xlsx / xls） -->
              <XlsxView v-else-if="consoleType === 'xlsx'"
                        class="flex-1 min-h-0"
                        :output="output"
                        :is-running="isRunning"
                        :execution-time="lastExecutionTime"
                        @clear="clearOutput"/>
            </div>
          </template>
        </ResizablePanels>
      </template>

      <!-- 仅编辑器：控制台未展开时占满 -->
      <div v-else class="h-full flex flex-col overflow-hidden">
        <EditorTabs :tabs="editorTabs" :active-id="activeTabId" @switch="switchTab" @close="handleCloseTab" @new="handleNewTab"
                          @close-others="closeOthers" @close-right="closeToRight" @move="moveTab" @copy-path="handleCopyPath"
                          @copy-relative="handleCopyRelativePath" @reveal-tree="revealInTree" @reveal-finder="revealInFinder" @toggle-pin="togglePin"/>
        <div v-if="!showViewer" class="bg-gray-100 dark:bg-gray-800 px-4 py-2 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between flex-shrink-0">
          <div class="flex items-center space-x-3 min-w-0 flex-1">
            <img :src="`/icons/${currentLanguage.replace(/\d+$/, '')}.svg`" class="w-5 h-5 flex-shrink-0" :alt="currentLanguage" @error="onIconError"/>
            <h2 class="text-sm font-medium text-gray-700 dark:text-gray-200 whitespace-nowrap flex-shrink-0">{{ getLanguageDisplayName(currentLanguage) }} {{ t('app.codeEditor') }}</h2>
            <template v-if="currentFilePath">
              <span class="text-gray-400 text-xs flex-shrink-0">·</span>
              <div class="min-w-0 overflow-x-auto [&::-webkit-scrollbar]:hidden [-ms-overflow-style:none] [scrollbar-width:none]">
                <Breadcrumbs :path="currentFilePath" :root-dir="rootDir" :dirty="isDirty" @reveal="revealInFinder" @open="smartOpen"/>
              </div>
            </template>
            <span v-else-if="currentFileName" class="text-xs text-gray-500 flex items-center whitespace-nowrap flex-shrink-0">
              · {{ currentFileName }}
              <span v-if="isDirty" class="ml-1 text-amber-500" :title="t('app.unsaved')">●</span>
            </span>
            <SqlSourceSelect v-if="currentLanguage === 'sql'" class="flex-shrink-0"/>
            <SchemaBrowser v-if="currentLanguage === 'sql'" class="flex-shrink-0" @preview="previewTable" @insert="insertAtCursor"/>
                  <AiSql v-if="currentLanguage === 'sql'" class="flex-shrink-0" @generated="insertAtCursor"/>
                  <ErDiagram v-if="currentLanguage === 'sql'" class="flex-shrink-0"/>
                  <TxnControl v-if="currentLanguage === 'sql'" class="flex-shrink-0" @notice="onTxnNotice"/>
          </div>

          <div class="flex items-center space-x-2 text-xs text-gray-500 whitespace-nowrap flex-shrink-0 pl-3">
            <span v-if="predicting" class="flex items-center gap-1 text-blue-500">
              <Sparkles class="w-3 h-3 animate-pulse"/> {{ t('app.aiPredicting') }}
            </span>
            <span v-else-if="ghostActive" class="text-blue-500">{{ t('app.ghostHint') }}</span>
            <span>{{ t('app.cursorPos', { line: cursorInfo.line, col: cursorInfo.col }) }}</span>
            <span v-if="cursorInfo.selLen">{{ t('app.selected') }} <strong>{{ cursorInfo.selLen }}</strong></span>
            <span><strong>{{ (code || '').length }}</strong> {{ t('app.chars') }}</span>
            <span><strong>{{ (code || '').split('\n').length }}</strong> {{ t('app.lines') }}</span>
            <IndentControl v-if="editorConfig" :config="editorConfig"/>
          </div>
        </div>
        <div class="flex-1 overflow-hidden relative">
          <CodeEditor v-model="code" class="h-full" :language="currentLanguage" :file-path="currentFilePath" :root-dir="rootDir" :editor-config="editorConfig" :key="editorConfigKey" @ready="editorView = $event"/>
          <LargeFileViewer v-if="showViewer && viewerFile"
                           :file-path="viewerFile.path"
                           :line-count="viewerFile.lineCount"
                           :size-bytes="viewerFile.sizeBytes"
                           @close="closeViewer"/>
          <InlineGenerate v-if="showGenerate" :language="currentLanguage" :selection="generateSelection" @insert="insertGeneratedCode" @close="showGenerate = false"/>
        </div>
      </div>
      </div>
    </div>

    <!-- 集成终端：停靠在底部，占据高度使上方编辑区自动收缩。
         首次打开后保持挂载，用 v-show 收起以保留会话；关闭所有标签才彻底卸载 -->
    <Terminal v-if="terminalMounted"
              ref="terminalRef"
              v-show="showTerminal"
              class="flex-shrink-0"
              :root-dir="rootDir"
              @collapse="showTerminal = false"
              @close="showTerminal = false; terminalMounted = false"/>

    <!-- 状态栏 -->
    <StatusBar class="flex-shrink-0" :env-info="envInfo" :is-loading="isLoadingEnvInfo" :execution-time="lastExecutionTime" :code-length="(code || '').length" @check-environment="refreshEnvInfo" @toggle-terminal="toggleTerminal" @toggle-problems="showDiagnostics = !showDiagnostics"/>

    <!-- 关于组件 -->
    <About v-if="showAbout" @close="closeAbout"/>

    <!-- 设置组件 -->
    <Settings v-if="showSettings" @close="onSettingsClose" @settings-changed="handleSettingsChanged"/>

    <!-- 更新组件 -->
    <Update v-if="showUpdate" @close="closeUpdate"/>

    <!-- 执行历史 -->
    <ExecutionHistory v-model:show="showHistory"
                      :supported-languages="supportedLanguages"
                      @restore="restoreHistoryItem"
                      @rerun="rerunHistoryItem"
                      @open-ai="openAiForExecution"/>

    <!-- 运行未保存文件询问 -->
    <Modal v-model:show="showRunPrompt" :title="t('app.runUnsavedTitle')" size="sm">
      <div class="space-y-4">
        <p class="text-sm text-gray-700 dark:text-gray-300">
          {{ t('app.runUnsavedPre') }}<strong>{{ currentFileName }}</strong>{{ t('app.runUnsavedPost') }}
        </p>
        <div class="flex justify-end space-x-2">
          <Button type="secondary" size="sm" @click="showRunPrompt = false">{{ t('app.cancel') }}</Button>
          <Button type="info" size="sm" @click="promptRunCopy">{{ t('app.runCopy') }}</Button>
          <Button size="sm" @click="promptSaveAndRun">{{ t('app.saveAndRun') }}</Button>
        </div>
      </div>
    </Modal>

    <!-- AI 助手 -->
    <AiAssistant v-if="showAi" :code="code" :language="currentLanguage" :execution-id="aiExecutionId" :error-context="aiErrorContext" :initial-prompt="aiInitialPrompt" :root-dir="rootDir" @close="showAi = false" @insert-code="applyAiCode"/>

    <!-- 文件夹内全局搜索 -->
    <SearchPanel v-if="showSearch && rootDir" :root-dir="rootDir" :extra-roots="extraRoots" :scope="searchScope" @open="openSearchResult" @replaced="reloadAffectedFiles" @close="showSearch = false"/>

    <!-- 快速打开文件 -->
    <QuickOpen v-if="showQuickOpen && rootDir"
               :root-dir="rootDir"
               :recent-paths="recentFiles"
               @select="smartOpen"
               @close="showQuickOpen = false"/>

    <!-- 命令面板 -->
    <CommandPalette v-if="showCommandPalette"
                    :commands="paletteCommands"
                    @close="showCommandPalette = false"/>

    <!-- 跳转到行 -->
    <GoToLine v-if="showGoToLine"
              :max-line="(code || '').split('\n').length"
              @go="gotoLine"
              @close="showGoToLine = false"/>

    <!-- 符号大纲 -->
    <Outline v-if="showOutline"
             :code="code"
             :language="currentLanguage"
             :current-line="cursorInfo.line"
             @go="gotoLine"
             @close="showOutline = false"/>

    <!-- 代码片段管理 -->
    <SnippetManager v-if="showSnippets" @close="showSnippets = false"/>

    <!-- 差异对比：当前 vs 已保存 -->
    <DiffView v-if="showDiff"
              :original="savedContent || ''"
              :modified="code"
              :file-name="currentFileName"
              @close="showDiff = false"/>

    <!-- 应用 AI 代码前的差异预览 -->
    <DiffView v-if="applyPreview"
              :original="code"
              :modified="applyPreview.modified"
              :title="t('app.aiPreviewTitle')"
              :subtitle="t('app.aiPreviewSubtitle')"
              :confirm-label="t('app.apply')"
              @confirm="confirmApplyAi"
              @close="applyPreview = null"/>

    <!-- 实时预览（Markdown / HTML）-->
    <PreviewPanel v-if="showPreview"
                  :content="code"
                  :language="currentLanguage"
                  :file-name="currentFileName"
                  @close="showPreview = false"/>

    <!-- 调试工具栏（会话进行中显示） -->
    <DebugToolbar/>

    <!-- 调试侧栏：调用栈 + 变量 -->
    <DebugPanel/>

    <!-- AI 代码操作（解释/重构/生成测试） -->
    <AiCodeAction v-if="aiCodeCtx" :language="currentLanguage" :code="aiCodeCtx.code" :action="aiCodeCtx.action"
                  @replace="onAiReplace" @insert="onAiInsert" @close="aiCodeCtx = null"/>

    <!-- .gitignore 模板 -->
    <GitIgnoreTemplates v-if="showGitignore && rootDir" :root-dir="rootDir" @close="showGitignore = false"/>

    <!-- 运行任务 -->
    <TaskRunner v-if="showTasks && rootDir" :root-dir="rootDir" @run="runTask" @close="showTasks = false"/>

    <!-- Git 源代码管理 -->
    <GitPanel v-if="showGit && rootDir"
              :root-dir="rootDir"
              @open="smartOpen"
              @refresh="refreshGitStatus"
              @close="showGit = false"/>

    <!-- LSP 问题面板 -->
    <DiagnosticsPanel v-if="showDiagnostics"
                      @go="(line, col) => gotoLine(line, col)"
                      @close="showDiagnostics = false"/>

    <!-- 编辑器 LSP 右键菜单 -->
    <div v-if="editorCtx.visible" class="fixed inset-0 z-50" @click="closeEditorCtx" @contextmenu.prevent="closeEditorCtx">
      <div ref="editorMenuRef"
           class="absolute bg-white dark:bg-gray-800 dark:text-gray-100 rounded-md shadow-lg border border-gray-200 dark:border-gray-700 py-1 text-sm min-w-[170px]"
           :style="{ top: `${editorCtx.y}px`, left: `${editorCtx.x}px` }"
           @click.stop>
        <template v-if="editorCtx.lsp">
          <button class="flex w-full items-center justify-between px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="runEditorCommand(runGotoDefinition)">
            <span>{{ t('app.gotoDef') }}</span><span class="text-gray-400 text-xs ml-6">F12</span>
          </button>
          <button class="flex w-full items-center justify-between px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="runEditorCommand(renameSymbol)">
            <span>{{ t('app.renameSymbol') }}</span><span class="text-gray-400 text-xs ml-6">F2</span>
          </button>
          <button class="flex w-full items-center justify-between px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="runEditorCommand(triggerCodeActions)">
            <span>{{ t('app.codeActionMenu') }}</span><span class="text-gray-400 text-xs ml-6">⌘.</span>
          </button>
          <div class="border-t border-gray-100 dark:border-gray-700 my-1"></div>
          <button class="flex w-full items-center justify-between px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="runEditorCommand(formatDocument)">
            <span>{{ t('app.formatDoc') }}</span><span class="text-gray-400 text-xs ml-6">⇧⌥F</span>
          </button>
          <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="runEditorCommand(formatSelection)">
            {{ t('app.formatSelection') }}
          </button>
        </template>
        <template v-if="canBlame">
          <div v-if="editorCtx.lsp" class="border-t border-gray-100 dark:border-gray-700 my-1"></div>
          <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="openBlame">
            {{ t('git.blame') }}
          </button>
          <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="openFileHistory">
            {{ t('git.fileHistory') }}
          </button>
          <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="() => { closeEditorCtx(); copyPermalink() }">
            {{ t('app.copyPermalinkShort') }}
          </button>
          <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="() => { closeEditorCtx(); openPermalink() }">
            {{ t('app.openOnRemote') }}
          </button>
        </template>
        <div v-if="editorCtx.lsp || canBlame" class="border-t border-gray-100 dark:border-gray-700 my-1"></div>
        <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="aiCodeAction('explain')">{{ t('aiCode.title.explain') }}</button>
        <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="aiCodeAction('refactor')">{{ t('aiCode.title.refactor') }}</button>
        <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="aiCodeAction('test')">{{ t('aiCode.title.test') }}</button>
        <div class="border-t border-gray-100 dark:border-gray-700 my-1"></div>
        <button class="w-full text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer" @click="sendToTerminal">
          {{ t('app.sendToTerminal') }}
        </button>
      </div>
    </div>

    <!-- Git Blame 逐行追溯 -->
    <BlameView v-if="blameInfo"
               :root-dir="blameInfo.root"
               :rel-path="blameInfo.rel"
               :file-name="blameInfo.name"
               @close="blameInfo = null"/>

    <!-- 文件提交历史 -->
    <GitLog v-if="fileHistory"
            :root-dir="fileHistory.root"
            :rel-path="fileHistory.rel"
            :file-name="fileHistory.name"
            @close="fileHistory = null"/>

    <!-- LSP 代码操作选择菜单 -->
    <div v-if="codeActionMenu.visible" class="fixed inset-0 z-50" @click="codeActionMenu.visible = false" @contextmenu.prevent="codeActionMenu.visible = false">
      <div class="absolute bg-white dark:bg-gray-800 dark:text-gray-100 rounded-md shadow-lg border border-gray-200 dark:border-gray-700 py-1 text-sm min-w-[200px] max-w-[420px] max-h-[320px] overflow-y-auto"
           :style="{ top: `${codeActionMenu.y}px`, left: `${codeActionMenu.x}px` }"
           @click.stop>
        <button v-for="(a, i) in codeActionMenu.actions" :key="i"
                class="block w-full truncate text-left px-3 py-1.5 hover:bg-gray-100 dark:hover:bg-gray-700 cursor-pointer"
                :title="a.title"
                @click="pickCodeAction(a)">
          {{ a.title }}
        </button>
      </div>
    </div>

    <!-- Toast 组件 -->
    <Toast/>
  </div>
</template>

<script setup lang="ts">
import {computed, nextTick, onMounted, onUnmounted, reactive, ref, shallowRef, watch} from 'vue'
import {useI18n} from 'vue-i18n'
import {debounce} from 'lodash-es'
import {formatDocument, formatSelection, renameSymbol} from 'codemirror-languageserver'
import {runGotoDefinition, lspSupportsLanguage, triggerCodeActions, applyCodeAction, formatDocumentAsync} from './editor/lspExtension'
import {dapSupportsLanguage} from './debug/dapClient'
import {ChevronRight, Code2, CornerDownRight, Eye, FolderOpen, GitBranch, GitCompare, History, ListChecks, ListTree, Maximize2, Monitor, Moon, PanelBottom, PanelLeft, PanelRight, Play, Plus, Save, Search, Settings as SettingsIcon, Sparkles, Sun, Terminal as TerminalIcon, WrapText, X} from 'lucide-vue-next'
import {ExecutionResult, LayoutMode, SplitDirection} from './types/app.ts'
import AppHeader from './components/AppHeader.vue'
import CodeEditor from './components/CodeEditor.vue'
import DiagnosticsPanel from './components/DiagnosticsPanel.vue'
import ConsoleOutput from './components/ConsoleOutput.vue'
import WebOutput from "./components/WebOutput.vue";
import JsonView from "./components/JsonView.vue";
import MarkdownView from "./components/MarkdownView.vue";
import XmlView from "./components/XmlView.vue";
import YamlView from "./components/YamlView.vue";
import SqlTableView from "./components/SqlTableView.vue";
import DataTableView from "./components/DataTableView.vue";
import XlsxView from "./components/XlsxView.vue";
import SqlSourceSelect from "./components/SqlSourceSelect.vue";
import SchemaBrowser from "./components/SchemaBrowser.vue";
import AiSql from "./components/AiSql.vue";
import StatusBar from './components/StatusBar.vue'
import About from './components/About.vue'
import Settings from './components/Settings.vue'
import Toast from './components/Toast.vue'
import ResizablePanels from './components/ResizablePanels.vue'
import {useToast} from './plugins/toast'

// Composables
import {useCodeExecution} from './composables/useCodeExecution'
import {useLanguageManager} from './composables/useLanguageManager'
import {useFileManager} from './composables/useFileManager'
import {useLanguageRegistry} from './composables/useLanguageRegistry'
import {useWorkspace} from './composables/useWorkspace'
import EditorTabs from './components/EditorTabs.vue'
import IndentControl from './components/IndentControl.vue'
import Sidebar from './components/Sidebar.vue'
import LargeFileViewer from './components/LargeFileViewer.vue'
import QuickOpen from './components/QuickOpen.vue'
import CommandPalette, {type PaletteCommand} from './components/CommandPalette.vue'
import DiffView from './components/DiffView.vue'
import PreviewPanel from './components/PreviewPanel.vue'
import BlameView from './components/BlameView.vue'
import GitLog from './components/GitLog.vue'
import GitPanel from './components/GitPanel.vue'
import TaskRunner from './components/TaskRunner.vue'
import GitIgnoreTemplates from './components/GitIgnoreTemplates.vue'
import DebugToolbar from './components/DebugToolbar.vue'
import DebugPanel from './components/DebugPanel.vue'
import AiCodeAction from './components/AiCodeAction.vue'
import ErDiagram from './components/ErDiagram.vue'
import TxnControl from './components/TxnControl.vue'
import {useSqlTxn} from './composables/useSqlTxn'
import GoToLine from './components/GoToLine.vue'
import Outline from './components/Outline.vue'
import SnippetManager from './components/SnippetManager.vue'
import Terminal from './components/Terminal.vue'
import Breadcrumbs from './components/Breadcrumbs.vue'
import {initSnippets} from './composables/useSnippets'
import {kvGet, kvGetJSON, kvSet, kvSetJSON} from './composables/useKvStore'
import {useDbConnections} from './composables/useDbConnections'
import {useAiConfig} from './composables/useAiConfig'
import {setGhost, clearGhostIn, ghostActive} from './editor/aiComplete'
import {cursorInfo} from './editor/cursorInfo'
import {computeDiffMarkers, setDiffMarkers} from './editor/diffGutter'
import {setBreakpointData} from './editor/breakpointGutter'
import {useDebug} from './composables/useDebug'
import AiAssistant from './components/AiAssistant.vue'
import InlineGenerate from './components/InlineGenerate.vue'
import SearchPanel from './components/SearchPanel.vue'
import {useTheme, type AppTheme} from './composables/useTheme'
import Modal from './ui/Modal.vue'
import Button from './ui/Button.vue'
import ExecutionHistory from './components/ExecutionHistory.vue'
import {open as openDialog} from '@tauri-apps/plugin-dialog'
import {open as openExternalUrl} from '@tauri-apps/plugin-shell'
import {invoke} from '@tauri-apps/api/core'
import {useEventManager} from './composables/useEventManager'
import {useShortcuts} from './composables/useShortcuts'
import {useAppState} from './composables/useAppState'
import {useEditorConfig} from './composables/useEditorConfig'
import Update from './components/Update.vue'

const toast = useToast()
const {t} = useI18n()
const showHistory = ref(false)

const {
  code,
  output,
  isRunning,
  isSuccess,
  lastExecutionTime,
  currentExecutionId,
  runCode,
  stopCode,
  clearOutput,
  handleRealtimeOutput,
  handleExecutionComplete,
  handleExecutionStopped,
  handleExecutionTimeout,
  handleExecutionError
} = useCodeExecution(toast)

const {
  currentLanguage,
  supportedLanguages,
  envInfo,
  isLoadingEnvInfo,
  getLanguageDisplayName,
  getCurrentConsoleType,
  getCurrentPluginConfig,
  handleLanguageChange,
  applyLanguage,
  refreshLanguageList,
  refreshEnvInfo,
  initialize
} = useLanguageManager(code, clearOutput, toast)

// 扩展名 ↔ 语言 注册表
const {build: buildLanguageRegistry, detectLanguage, getCandidates} = useLanguageRegistry()

// 本地文件管理（打开/保存/另存为）
const getDefaultFileName = () => {
  const ext = getCurrentPluginConfig()?.extension || currentLanguage.value || 'txt'
  return `${t('app.untitled')}.${ext}`
}

// 文件状态（提升到此层，供文件管理与多标签工作区共享）
const currentFilePath = ref<string | null>(null)
const savedContent = ref<string | null>(null)
const restoreFile = (filePath: string | null, saved: string | null) => {
  currentFilePath.value = filePath
  savedContent.value = saved
}

// 多标签工作区
const {
  tabs: editorTabs,
  activeTabId,
  switchTab,
  newTab,
  closeTab,
  closeOthers,
  closeToRight,
  reopenClosed,
  togglePin,
  restorePinned,
  moveTab,
  updateTabPath,
  detachTabPath,
  isActiveReusableScratch,
  initFirstTab
} = useWorkspace({code, currentLanguage, applyLanguage, currentFilePath, savedContent, restoreFile})

// 打开文件后按扩展名自动切换语言（不改动已载入的内容、不解除文件关联）
const handleFileOpened = (filePath: string) => {
  // 优先保持当前语言：当前引擎已匹配该扩展名时不切换（如已在某 JS 引擎上打开 .js）
  const detected = detectLanguage(filePath, currentLanguage.value)
  if (detected && detected !== currentLanguage.value) {
    applyLanguage(detected)
    // 同扩展名对应多个引擎时，提示可手动切换
    if (getCandidates(filePath).length > 1) {
      toast.info(t('app.multiEngine', { name: getLanguageDisplayName(detected) }))
    }
  }
}

const {
  currentFileName,
  isDirty,
  pickFile,
  openPath,
  saveFile,
  saveFileAs,
  resetFile
} = useFileManager({
  code,
  toast,
  getDefaultFileName,
  currentFilePath,
  savedContent,
  // 打开文件时若当前不是空白草稿，则在新标签页打开
  onBeforeLoad: () => {
    if (!isActiveReusableScratch()) {
      newTab({language: currentLanguage.value})
    }
  },
  onOpened: handleFileOpened,
  getMaxFileSizeMb: () => editorConfig.value?.max_open_file_size
})

// 手动切换语言（下拉框）：替换为模板并解除文件关联
const onLanguageChange = (language: string) => {
  handleLanguageChange(language)
  resetFile()
}

const handleNewTab = () => newTab({language: currentLanguage.value, code: ''})
const handleCloseTab = (id: string) => closeTab(id, {language: currentLanguage.value})
const handleReopenClosed = () => {
  const reopened = reopenClosed()
  if (!reopened) {
    toast.info(t('app.noClosedTab'))
  }
}

const handleCopyPath = async (path: string) => {
  try {
    await navigator.clipboard.writeText(path)
    toast.success(t('app.pathCopied'))
  }
  catch (error) {
    toast.error(t('app.copyFailed') + error)
  }
}
// 生成当前文件指定行的远程仓库永久链接
const buildPermalink = async (): Promise<string | null> => {
  if (!rootDir.value || !currentFilePath.value) {
    toast.info(t('app.noFileToReveal'))
    return null
  }
  const root = rootDir.value
  const p = currentFilePath.value
  if (!(p === root || p.startsWith(root + '/') || p.startsWith(root + '\\'))) {
    toast.info(t('app.permalinkOutside'))
    return null
  }
  const rel = p.slice(root.length).replace(/^[\\/]/, '')
  try {
    return await invoke<string>('git_permalink', {root, relPath: rel, line: cursorInfo.value.line})
  }
  catch (error) {
    toast.error(t('app.permalinkFailed') + ': ' + error)
    return null
  }
}
const copyPermalink = async () => {
  const url = await buildPermalink()
  if (url) {
    await navigator.clipboard.writeText(url)
    toast.success(t('app.permalinkCopied'))
  }
}
const openPermalink = async () => {
  const url = await buildPermalink()
  if (url) {
    await openExternalUrl(url).catch((e) => toast.error(t('app.permalinkFailed') + ': ' + e))
  }
}

const handleCopyRelativePath = (path: string) => {
  const root = rootDir.value
  const rel = root && (path === root || path.startsWith(root + '/') || path.startsWith(root + '\\'))
      ? path.slice(root.length).replace(/^[\\/]/, '')
      : path
  handleCopyPath(rel)
}

// ===== 侧栏 / 文件夹 =====
const rootDir = ref<string | null>(null)
const sidebarVisible = ref(kvGet('sidebar-visible') === 'true')
const sidebarWidth = ref(Number(kvGet('sidebar-width')) || 240)

// 最近打开的文件夹
const RECENT_FOLDERS_KEY = 'recent-folders'
const LAST_ROOT_KEY = 'last-root-dir'
const recentFolders = ref<string[]>(kvGetJSON<string[]>(RECENT_FOLDERS_KEY, []))

// 最近打开的文件（用于 Cmd+P 优先展示）
const RECENT_FILES_KEY = 'recent-files'
const recentFiles = ref<string[]>(kvGetJSON<string[]>(RECENT_FILES_KEY, []))
const addRecentFile = (path: string) => {
  const list = [path, ...recentFiles.value.filter(p => p !== path)].slice(0, 30)
  recentFiles.value = list
  kvSetJSON(RECENT_FILES_KEY, list)
}

// 记住打开的文件夹（去重、置顶、最多 8 个），并记录为上次文件夹
const rememberFolder = (path: string) => {
  const list = [path, ...recentFolders.value.filter(p => p !== path)].slice(0, 8)
  recentFolders.value = list
  kvSetJSON(RECENT_FOLDERS_KEY, list)
  kvSet(LAST_ROOT_KEY, path)
}

const openFolderPath = (path: string) => {
  rootDir.value = path
  sidebarVisible.value = true
  rememberFolder(path)
  // 打开新文件夹视为新工作区，清空额外挂载的根
  extraRoots.value = []
  kvSetJSON(WORKSPACE_EXTRA_KEY, extraRoots.value)
}

// ===== 多根工作区（E3，phase 1）：额外挂载的文件夹（Git/搜索仍走主根 rootDir）=====
const WORKSPACE_EXTRA_KEY = 'workspace-extra-roots'
const extraRoots = ref<string[]>(kvGetJSON<string[]>(WORKSPACE_EXTRA_KEY, []))
const addWorkspaceFolder = async () => {
  const selected = await openDialog({directory: true, multiple: false})
  if (selected && typeof selected === 'string' && selected !== rootDir.value && !extraRoots.value.includes(selected)) {
    extraRoots.value = [...extraRoots.value, selected]
    kvSetJSON(WORKSPACE_EXTRA_KEY, extraRoots.value)
  }
}
const removeWorkspaceFolder = (path: string) => {
  extraRoots.value = extraRoots.value.filter(p => p !== path)
  kvSetJSON(WORKSPACE_EXTRA_KEY, extraRoots.value)
}

// ===== 标签会话持久化 =====
const SESSION_TABS_KEY = 'session-tabs'

const persistSession = () => {
  const paths = editorTabs.value.map(t => t.filePath).filter((p): p is string => !!p)
  const pinned = editorTabs.value.filter(t => t.pinned && t.filePath).map(t => t.filePath as string)
  const activePath = editorTabs.value.find(t => t.id === activeTabId.value)?.filePath || null
  kvSetJSON(SESSION_TABS_KEY, {paths, pinned, activePath})
}

// 标签集合/文件/激活项/置顶变化时持久化（不含正文编辑，避免频繁写入）
watch(
    () => editorTabs.value.map(t => (t.pinned ? '*' : '') + (t.filePath || '')).join('|') + '#' + activeTabId.value,
    () => persistSession()
)

// 启动时恢复上次打开的文件标签（仅已保存且可读的文本文件）
const restoreSession = async () => {
  const saved = kvGetJSON<{ paths: string[], pinned?: string[], activePath: string | null } | null>(SESSION_TABS_KEY, null)
  if (!saved || !saved.paths?.length) {
    return
  }

  const limitBytes = (editorConfig.value?.max_open_file_size ?? 5) * 1024 * 1024
  for (const p of saved.paths) {
    try {
      const meta = await invoke<{ size_bytes: number, is_text: boolean }>('get_text_file_meta', {path: p})
      if (meta.is_text && meta.size_bytes <= limitBytes) {
        await openPath(p)
      }
    }
    catch {
      // 跳过已删除/无法读取的文件
    }
  }

  if (saved.pinned?.length) {
    restorePinned(saved.pinned)
  }

  if (saved.activePath) {
    const t = editorTabs.value.find(tab => tab.filePath === saved.activePath)
    if (t) {
      switchTab(t.id)
    }
  }
}

watch(sidebarVisible, (v) => kvSet('sidebar-visible', String(v)))

// 拖拽改变侧栏宽度
let resizeStartX = 0
let resizeStartWidth = 0
const onSidebarResize = (e: MouseEvent) => {
  const w = resizeStartWidth + (e.clientX - resizeStartX)
  sidebarWidth.value = Math.max(160, Math.min(600, w))
}
const stopSidebarResize = () => {
  document.removeEventListener('mousemove', onSidebarResize)
  document.removeEventListener('mouseup', stopSidebarResize)
  document.body.style.userSelect = ''
  document.body.style.cursor = ''
  kvSet('sidebar-width', String(sidebarWidth.value))
}
const startSidebarResize = (e: MouseEvent) => {
  e.preventDefault()
  resizeStartX = e.clientX
  resizeStartWidth = sidebarWidth.value
  document.addEventListener('mousemove', onSidebarResize)
  document.addEventListener('mouseup', stopSidebarResize)
  document.body.style.userSelect = 'none'
  document.body.style.cursor = 'col-resize'
}

const toggleSidebar = () => {
  sidebarVisible.value = !sidebarVisible.value
}

const openFolder = async () => {
  const selected = await openDialog({directory: true, multiple: false})
  if (selected && typeof selected === 'string') {
    openFolderPath(selected)
  }
}

// 只读大文件查看器状态
const showViewer = ref(false)
const viewerFile = ref<{ path: string, lineCount: number, sizeBytes: number } | null>(null)

// 二进制数据文件（Excel 等）：切到对应语言并在控制台用数据视图展示，不进代码编辑器
const openDataFile = (filePath: string, lang: string) => {
  addRecentFile(filePath)
  applyLanguage(lang)
  currentFilePath.value = filePath
  output.value = filePath
  showConsole.value = true
  consoleType.value = getCurrentConsoleType()
}

// 按文件大小决定：可编辑打开 / 只读查看
const smartOpen = async (filePath: string) => {
  try {
    // Excel 等二进制数据文件：直接路由到数据视图，不当文本打开
    if (/\.(xlsx|xls)$/i.test(filePath)) {
      openDataFile(filePath, 'xlsx')
      return
    }

    const meta = await invoke<{ size_bytes: number, line_count: number, is_text: boolean }>('get_text_file_meta', {path: filePath})
    if (!meta.is_text) {
      toast.error(t('app.notTextFile'))
      return
    }

    const limitBytes = (editorConfig.value?.max_open_file_size ?? 5) * 1024 * 1024
    if (meta.size_bytes > limitBytes) {
      // 超过可编辑上限 → 只读查看器
      viewerFile.value = {path: filePath, lineCount: meta.line_count, sizeBytes: meta.size_bytes}
      showViewer.value = true
      return
    }

    addRecentFile(filePath)

    // 已打开则切换到对应标签，否则在新标签打开
    const existing = editorTabs.value.find(t => t.filePath === filePath)
    if (existing) {
      switchTab(existing.id)
      return
    }
    await openPath(filePath)
  }
  catch (error) {
    toast.error(t('app.openFailed') + error)
  }
}

const handleOpenFileClick = async () => {
  const path = await pickFile()
  if (path) {
    await smartOpen(path)
  }
}

// AI 助手抽屉（绑定的执行 id：工具栏打开取最近一次运行，历史面板打开取指定运行）
const showAi = ref(false)
const aiExecutionId = ref<number | null>(null)
// 失败运行的报错上下文，供"分析报错"快捷动作
const aiErrorContext = ref<{ code: string, error: string } | null>(null)
// 打开 AI 时的初始提示（解释/生成测试等一次性动作）
const aiInitialPrompt = ref<string | null>(null)

const combinedOutput = (item: ExecutionResult) =>
    [item.stdout?.trim(), item.stderr?.trim()].filter(Boolean).join('\n\n')

const handleShowAi = () => {
  aiExecutionId.value = currentExecutionId.value
  // 最近一次运行失败则带上报错
  aiErrorContext.value = currentExecutionId.value != null && !isSuccess.value && output.value
      ? {code: code.value, error: output.value}
      : null
  aiInitialPrompt.value = null
  showAi.value = true
}

// 取选中文本，无选中则用整篇代码
const selectedOrAll = (): string => {
  const view = editorView.value
  if (view) {
    const {from, to} = view.state.selection.main
    if (from !== to) {
      return view.state.sliceDoc(from, to)
    }
  }
  return code.value
}

// 以一次性提示打开 AI（临时会话，不关联执行）
const openAiWithPrompt = (prompt: string) => {
  aiExecutionId.value = null
  aiErrorContext.value = null
  aiInitialPrompt.value = prompt
  showAi.value = true
}

const explainCode = () => {
  const snippet = selectedOrAll()
  if (!snippet.trim()) {
    toast.info(t('app.noCodeExplain'))
    return
  }
  openAiWithPrompt(`请解释下面这段 ${currentLanguage.value} 代码的作用与关键逻辑，用中文简洁说明：\n\n\`\`\`${currentLanguage.value}\n${snippet}\n\`\`\``)
}

const generateTests = () => {
  const snippet = selectedOrAll()
  if (!snippet.trim()) {
    toast.info(t('app.noCodeTest'))
    return
  }
  openAiWithPrompt(`请为下面这段 ${currentLanguage.value} 代码生成单元测试，覆盖主要分支与边界情况，使用该语言常用的测试框架，只输出测试代码：\n\n\`\`\`${currentLanguage.value}\n${snippet}\n\`\`\``)
}

// AI 格式化/整理代码：请求 AI 返回整理后的完整代码，再走应用前差异预览确认
const formatWithAi = async () => {
  reloadAiCfg()
  if (!aiActive.value.apiKey) {
    toast.warning(t('app.needApiKey'))
    return
  }
  const src = code.value
  if (!src.trim()) {
    return
  }
  toast.info(t('app.aiFormatting'))
  try {
    const res = await invoke<string>('ai_chat', {
      provider: aiActive.value.provider,
      baseUrl: aiActive.value.baseUrl,
      apiKey: aiActive.value.apiKey,
      model: aiActive.value.model,
      system: '你是代码格式化与整理工具。只输出整理后的完整代码，保持逻辑与行为不变，规范缩进与风格，不要解释、不要使用代码块标记。',
      messages: [{role: 'user', content: `语言：${currentLanguage.value}\n请整理/格式化下面的代码：\n${src}`}]
    })
    const formatted = cleanCompletion(res)
    if (!formatted || formatted === src) {
      toast.info(t('app.noAdjust'))
      return
    }
    applyAiCode(formatted)
  }
  catch (error) {
    toast.error(t('app.formatFailed') + error)
  }
}

const openAiForExecution = (item: ExecutionResult) => {
  aiExecutionId.value = item.id ?? null
  aiErrorContext.value = item.success
      ? null
      : {code: item.code, error: combinedOutput(item) || '(无输出)'}
  showAi.value = true
}

// ===== 保存时格式化（走 LSP textDocument/formatting）=====
const formatOnSave = ref(kvGet('format-on-save') === 'true')
const toggleFormatOnSave = () => {
  formatOnSave.value = !formatOnSave.value
  kvSet('format-on-save', String(formatOnSave.value))
  toast.info(formatOnSave.value ? t('app.formatOnSaveOn') : t('app.formatOnSaveOff'))
}

// ===== AI 代码预测（幽灵补全，Tab 接受）=====
const {active: aiActive, reload: reloadAiCfg} = useAiConfig()
const aiCompletion = ref(kvGet('ai-completion') === 'true')

const toggleAiCompletion = () => {
  aiCompletion.value = !aiCompletion.value
  kvSet('ai-completion', String(aiCompletion.value))
  if (!aiCompletion.value) {
    clearGhostIn(editorView.value)
    toast.info(t('app.aiPredictOff'))
    return
  }
  // 开启时若未配置 API Key，明确提示（否则预测会静默不工作）
  reloadAiCfg()
  if (!aiActive.value.apiKey) {
    toast.warning(t('app.aiPredictOnNoKey'))
  }
  else {
    toast.info(t('app.aiPredictOn'))
  }
}

// 清洗补全结果：去掉代码块围栏与开头多余换行
const cleanCompletion = (raw: string): string => {
  let s = raw.replace(/^```[a-z]*\n?/i, '').replace(/```\s*$/i, '')
  s = s.replace(/^\n+/, '')
  return s
}

let predictNonce = 0
let predictInflight = 0
// 是否正在请求 AI 预测（用于状态提示）
const predicting = ref(false)
const requestPrediction = async () => {
  if (!aiCompletion.value || isRunning.value) {
    return
  }
  const view = editorView.value
  if (!view) {
    return
  }
  reloadAiCfg()
  if (!aiActive.value.apiKey) {
    return
  }
  const sel = view.state.selection.main
  if (!sel.empty) {
    return
  }
  const pos = sel.head
  const prefix = view.state.sliceDoc(0, pos)
  if (!prefix.trim()) {
    return
  }
  const suffix = view.state.sliceDoc(pos)
  const nonce = ++predictNonce
  predictInflight++
  predicting.value = true
  try {
    const res = await invoke<string>('ai_chat', {
      provider: aiActive.value.provider,
      baseUrl: aiActive.value.baseUrl,
      apiKey: aiActive.value.apiKey,
      model: aiActive.value.model,
      system: '你是代码自动补全引擎。只输出应插入在光标处的后续代码，不要解释、不要重复已有代码、不要使用代码块标记。若无合适补全则输出空。',
      messages: [{
        role: 'user',
        content: `语言：${currentLanguage.value}\n光标前代码：\n${prefix}\n\n光标后代码：\n${suffix}\n\n请仅输出应插入光标处的后续代码：`
      }]
    })
    // 丢弃过期结果或光标已移动的情况
    if (nonce !== predictNonce) {
      return
    }
    const v = editorView.value
    if (!v || v.state.selection.main.head !== pos) {
      return
    }
    const text = cleanCompletion(res)
    if (text) {
      v.dispatch({effects: setGhost.of({text, pos})})
    }
  }
  catch {
    // 静默失败，不打扰编辑
  }
  finally {
    predictInflight--
    if (predictInflight === 0) {
      predicting.value = false
    }
  }
}
const requestPredictionDebounced = debounce(requestPrediction, 600)
watch(code, () => {
  if (aiCompletion.value) {
    requestPredictionDebounced()
  }
})

// 应用 AI 代码块：先弹出差异预览，确认后再替换（避免直接覆盖）
const applyPreview = ref<{ modified: string } | null>(null)
const applyAiCode = (codeText: string) => {
  applyPreview.value = {modified: codeText}
}
const confirmApplyAi = () => {
  if (applyPreview.value) {
    code.value = applyPreview.value.modified
    applyPreview.value = null
    toast.success(t('app.aiCodeApplied'))
  }
}

// 当前 CodeMirror view（用于在光标处插入生成的代码）
// shallowRef：EditorView 是含 LSP client/plugin 等大量可变内部状态的对象，
// 绝不能被 Vue 深度响应式代理。否则 watch(editorView) 会在 view 内部每次 mutation 时触发，
// 与 applyDiffMarkers 的 dispatch 形成 mutation→watch→dispatch→mutation 无限循环导致整页卡死
// （LSP 接入后 client 持续 mutate 会立刻触发该循环）。
const editorView = shallowRef<any>(null)

// AI 自然语言生成 / 选区改写
const showGenerate = ref(false)
const generateSelection = ref('')
const openGenerate = () => {
  const view = editorView.value
  generateSelection.value = view
      ? view.state.sliceDoc(view.state.selection.main.from, view.state.selection.main.to)
      : ''
  showGenerate.value = true
}

// 在光标处插入/替换选区为生成的代码
const insertGeneratedCode = (text: string) => {
  const view = editorView.value
  if (view) {
    const sel = view.state.selection.main
    view.dispatch({
      changes: {from: sel.from, to: sel.to, insert: text},
      selection: {anchor: sel.from + text.length}
    })
    view.focus()
  }
  else {
    code.value = text
  }
}

// 文件夹内全局搜索（Cmd+Shift+F）
const showSearch = ref(false)
const searchScope = ref<string | null>(null)
const openSearch = () => {
  if (!rootDir.value) {
    toast.info(t('app.openFolderFirst'))
    return
  }
  searchScope.value = null
  showSearch.value = true
}
// 来自文件树「在文件夹中搜索」：限定搜索范围为该目录
const openSearchInFolder = (path: string) => {
  if (!rootDir.value) {
    return
  }
  searchScope.value = path
  showSearch.value = true
}

const gotoLine = (line: number, character?: number) => {
  const view = editorView.value
  if (!view) {
    return
  }
  const target = Math.max(1, Math.min(line, view.state.doc.lines))
  const l = view.state.doc.line(target)
  // 带列号时精确定位到列（夹在行内），否则定位到行首
  const anchor = character != null ? Math.min(l.from + character, l.to) : l.from
  view.dispatch({selection: {anchor}, scrollIntoView: true})
  view.focus()
}

// 跳转到行（Cmd+G）
const showGoToLine = ref(false)
const openGoToLine = () => {
  showGoToLine.value = true
}

// 符号大纲（Cmd+Shift+O）
const showOutline = ref(false)
const openOutline = () => {
  showOutline.value = true
}

// 在文件树中定位当前文件
const revealRequest = ref<{ path: string, n: number } | null>(null)
let revealSeq = 0
const revealInTree = (path?: string) => {
  const target = path ?? currentFilePath.value
  if (!target) {
    toast.info(t('app.noFileToReveal'))
    return
  }
  sidebarVisible.value = true
  revealRequest.value = {path: target, n: ++revealSeq}
}

// 切换文件时自动在文件树中定位（仅当侧栏已打开，避免频繁强开侧栏打扰）
const autoRevealTree = ref(kvGet('auto-reveal-tree') === 'true')
const toggleAutoReveal = () => {
  autoRevealTree.value = !autoRevealTree.value
  kvSet('auto-reveal-tree', String(autoRevealTree.value))
  toast.info(autoRevealTree.value ? t('app.autoRevealOn') : t('app.autoRevealOff'))
  if (autoRevealTree.value && sidebarVisible.value && currentFilePath.value) {
    revealRequest.value = {path: currentFilePath.value, n: ++revealSeq}
  }
}
watch(currentFilePath, (p) => {
  if (autoRevealTree.value && sidebarVisible.value && p) {
    revealRequest.value = {path: p, n: ++revealSeq}
  }
})

// 代码片段管理
const showSnippets = ref(false)

// 语言图标缺失时回落到通用文本图标
const onIconError = (e: Event) => {
  const t = e.target as HTMLImageElement
  if (!t.src.endsWith('/icons/text.svg')) {
    t.src = '/icons/text.svg'
  }
}

// 面包屑点击：在系统文件管理器中显示该路径
const revealInFinder = (path: string) => {
  invoke('reveal_path', {path}).catch((error) => toast.error(t('app.openFailed') + error))
}

// 集成终端：首次打开后保持挂载（保留会话），仅切换显示
const showTerminal = ref(false)
const terminalMounted = ref(false)
const terminalRef = ref<any>(null)
const toggleTerminal = () => {
  if (showTerminal.value) {
    showTerminal.value = false
  }
  else {
    terminalMounted.value = true
    showTerminal.value = true
  }
}

// ===== .gitignore 模板（F2）=====
const showGitignore = ref(false)
const openGitignore = () => {
  if (!rootDir.value) {
    toast.info(t('app.openFolderFirst'))
    return
  }
  showGitignore.value = true
}

// ===== 运行任务（B4）：在集成终端中执行预设命令 =====
const showTasks = ref(false)
const openTasks = () => {
  if (!rootDir.value) {
    toast.info(t('app.openFolderFirst'))
    return
  }
  showTasks.value = true
}
const runTask = async (command: string) => {
  terminalMounted.value = true
  showTerminal.value = true
  await nextTick()
  terminalRef.value?.runCommand(command)
}

// B1-P3：开始调试当前文件（需已保存 + 语言可调试）
const startDebug = async () => {
  const path = currentFilePath.value
  if (!path) {
    toast.info(t('debug.saveFirst'))
    return
  }
  const lang = currentLanguage.value
  if (!dapSupportsLanguage(lang)) {
    toast.info(t('debug.langUnsupported'))
    return
  }
  // 适配器可用性检查（Python 含 debugpy 模块校验），不可用则给安装提示
  let ok = false
  try {
    ok = await invoke<boolean>('dap_available', {language: lang})
  }
  catch {
    ok = false
  }
  if (!ok) {
    const hint = lang === 'go' ? t('debug.installGo')
      : (lang === 'rust' || lang === 'c' || lang === 'cpp') ? t('debug.installLldb')
        : t('debug.installPython')
    toast.error(hint)
    return
  }
  try {
    await debug.startSession({filePath: path, language: lang, cwd: rootDir.value})
  }
  catch (error) {
    toast.error(t('debug.startFailed') + ': ' + error)
  }
}

// B2：按项目类型识别测试命令（读取根目录顶层标记文件）
const detectTestCommand = async (): Promise<string | null> => {
  const root = rootDir.value
  if (!root) {
    return null
  }
  let names: string[] = []
  try {
    const nodes = await invoke<{ name: string; is_dir: boolean }[]>('read_directory_tree', {path: root})
    names = nodes.map(n => n.name)
  }
  catch {
    return null
  }
  const has = (n: string) => names.includes(n)
  if (has('Cargo.toml')) {
    return 'cargo test'
  }
  if (has('go.mod')) {
    return 'go test ./...'
  }
  if (has('package.json')) {
    if (has('pnpm-lock.yaml')) {
      return 'pnpm test'
    }
    if (has('yarn.lock')) {
      return 'yarn test'
    }
    return 'npm test'
  }
  if (has('pyproject.toml') || has('pytest.ini') || has('setup.py') || has('tox.ini')) {
    return 'pytest'
  }
  if (has('pom.xml')) {
    return 'mvn test'
  }
  if (has('build.gradle') || has('build.gradle.kts')) {
    return 'gradle test'
  }
  if (has('Gemfile')) {
    return 'bundle exec rake test'
  }
  if (has('composer.json')) {
    return 'composer test'
  }
  if (has('Makefile')) {
    return 'make test'
  }
  return null
}
const runTests = async () => {
  if (!rootDir.value) {
    toast.info(t('app.openFolderFirst'))
    return
  }
  const cmd = await detectTestCommand()
  if (!cmd) {
    toast.info(t('app.testCmdNotFound'))
    return
  }
  await runTask(cmd)
}

// C2：对选区（无选区则整篇）执行 AI 操作：解释 / 重构 / 生成测试
const aiCodeCtx = ref<{action: 'explain' | 'refactor' | 'test'; code: string; from: number; to: number} | null>(null)
const aiCodeAction = (action: 'explain' | 'refactor' | 'test') => {
  closeEditorCtx()
  const view = editorView.value
  if (!view) {
    return
  }
  const sel = view.state.selection.main
  let from = sel.from
  let to = sel.to
  let code = sel.empty ? '' : view.state.sliceDoc(from, to)
  if (!code.trim()) {
    code = view.state.doc.toString()
    from = 0
    to = view.state.doc.length
  }
  if (!code.trim()) {
    toast.info(t('aiCode.noCode'))
    return
  }
  aiCodeCtx.value = {action, code, from, to}
}
const onAiReplace = (code: string) => {
  const view = editorView.value
  const ctx = aiCodeCtx.value
  if (view && ctx) {
    view.dispatch({changes: {from: ctx.from, to: ctx.to, insert: code}})
  }
}
const onAiInsert = (code: string) => {
  const view = editorView.value
  const ctx = aiCodeCtx.value
  if (view && ctx) {
    view.dispatch({changes: {from: ctx.to, insert: `\n\n${code}\n`}})
  }
}

// B3：发送选区（无选区则当前行）到集成终端，用于 REPL 式交互
const sendToTerminal = async () => {
  closeEditorCtx()
  const view = editorView.value
  if (!view) {
    return
  }
  const sel = view.state.selection.main
  const text = sel.empty
    ? view.state.doc.lineAt(sel.head).text
    : view.state.sliceDoc(sel.from, sel.to)
  if (!text.trim()) {
    return
  }
  await runTask(text)
}

const openSearchResult = async (path: string, line: number) => {
  showSearch.value = false
  await smartOpen(path)
  await nextTick()
  gotoLine(line)
}

// LSP 跨文件跳转定义：编辑器扩展派发 lsp:open-location，这里打开目标文件并定位
const onLspOpenLocation = async (e: Event) => {
  const detail = (e as CustomEvent).detail as {path: string; line: number; character?: number}
  if (!detail?.path) {
    return
  }
  await smartOpen(detail.path)
  await nextTick()
  gotoLine(detail.line, detail.character)
}

// LSP 问题面板显隐
const showDiagnostics = ref(false)

// ===== 编辑器 LSP 右键菜单（跳转定义 / 重命名 / 格式化）=====
const editorCtx = reactive({visible: false, x: 0, y: 0, lsp: false})
const editorMenuRef = ref<HTMLElement | null>(null)
const closeEditorCtx = () => {
  editorCtx.visible = false
}

// Git Blame：当前文件在已打开文件夹内时可用
const blameInfo = ref<{ root: string; rel: string; name: string } | null>(null)
const canBlame = computed(() => !!rootDir.value && !!currentFilePath.value && currentFilePath.value.startsWith(rootDir.value))
const openBlame = () => {
  editorCtx.visible = false
  const root = rootDir.value
  const path = currentFilePath.value
  if (!root || !path) {
    return
  }
  const rel = path.slice(root.length).replace(/^[\\/]/, '')
  blameInfo.value = {root, rel, name: rel.split(/[\\/]/).pop() || rel}
}

// 文件提交历史
const fileHistory = ref<{ root: string; rel: string; name: string } | null>(null)
const openFileHistory = () => {
  editorCtx.visible = false
  const root = rootDir.value
  const path = currentFilePath.value
  if (!root || !path) {
    return
  }
  const rel = path.slice(root.length).replace(/^[\\/]/, '')
  fileHistory.value = {root, rel, name: rel.split(/[\\/]/).pop() || rel}
}
const onEditorContext = async (e: MouseEvent) => {
  const target = e.target as HTMLElement | null
  const lsp = lspSupportsLanguage(currentLanguage.value) && !!editorView.value
  // 在编辑器内容区，且支持 LSP 或可 Blame 时弹出
  if (!target?.closest('.cm-content') || (!lsp && !canBlame.value)) {
    return
  }
  editorCtx.lsp = lsp
  e.preventDefault()
  const view = editorView.value
  if (view) {
    const cur = view.state.selection.main
    const pos = view.posAtCoords({x: e.clientX, y: e.clientY})
    // 仅在无选区、或右键点在选区之外时才移动光标；点在选区内则保留选区（不清除高亮）
    const insideSel = !cur.empty && pos != null && pos >= cur.from && pos <= cur.to
    if (pos != null && !insideSel) {
      view.dispatch({selection: {anchor: pos}})
    }
  }
  // 先按光标位置弹出，渲染后测量真实尺寸再夹取到视口内（菜单项数量可变，避免贴底/贴右裁切）
  editorCtx.x = e.clientX
  editorCtx.y = e.clientY
  editorCtx.visible = true
  await nextTick()
  const el = editorMenuRef.value
  if (el) {
    const r = el.getBoundingClientRect()
    const margin = 8
    if (editorCtx.x + r.width > window.innerWidth) {
      editorCtx.x = Math.max(margin, window.innerWidth - r.width - margin)
    }
    if (editorCtx.y + r.height > window.innerHeight) {
      editorCtx.y = Math.max(margin, window.innerHeight - r.height - margin)
    }
  }
}
const runEditorCommand = (cmd: (v: any) => boolean) => {
  closeEditorCtx()
  // 不在此处 focus 编辑器：重命名会弹出需要焦点的内联输入框
  if (editorView.value) {
    cmd(editorView.value)
  }
}

// ===== LSP 代码操作选择菜单 =====
const codeActionMenu = reactive<{visible: boolean; x: number; y: number; actions: any[]}>({
  visible: false, x: 0, y: 0, actions: []
})
// 编辑器扩展请求完成后派发 lsp:code-actions：有结果则弹菜单，无则提示
const onLspCodeActions = (e: Event) => {
  const detail = (e as CustomEvent).detail as {actions: any[]; x: number; y: number}
  const actions = detail?.actions ?? []
  if (!actions.length) {
    toast.info(t('app.noCodeAction'))
    return
  }
  codeActionMenu.actions = actions
  codeActionMenu.x = Math.min(detail.x, window.innerWidth - 430)
  codeActionMenu.y = Math.min(detail.y, window.innerHeight - 340)
  codeActionMenu.visible = true
}
const pickCodeAction = async (action: any) => {
  codeActionMenu.visible = false
  if (!editorView.value) {
    return
  }
  try {
    const {otherFiles} = await applyCodeAction(editorView.value, action)
    if (otherFiles > 0) {
      toast.info(t('app.actionOtherFiles', { n: otherFiles }))
    }
  }
  catch (err) {
    toast.error(t('app.applyActionFailed') + err)
  }
}

// 全局替换后：刷新涉及到的已打开标签（保留有未保存修改的标签）
const reloadAffectedFiles = async (paths: string[]) => {
  const set = new Set(paths)
  for (const tab of editorTabs.value) {
    if (!tab.filePath || !set.has(tab.filePath)) {
      continue
    }
    // 有未保存修改则不覆盖，避免丢失用户编辑
    const dirty = tab.savedContent !== null && tab.code !== tab.savedContent
    if (dirty) {
      continue
    }
    try {
      const content = await invoke<string>('read_file_text', {
        path: tab.filePath,
        maxSizeMb: editorConfig.value?.max_open_file_size
      })
      if (tab.id === activeTabId.value) {
        code.value = content
        savedContent.value = content
      }
      else {
        tab.code = content
        tab.savedContent = content
      }
    }
    catch {
      // 跳过无法读取的文件
    }
  }
}

// 快速打开（Cmd+P）
const showQuickOpen = ref(false)
const openQuickOpen = () => {
  if (!rootDir.value) {
    toast.info(t('app.openFolderFirst'))
    return
  }
  showQuickOpen.value = true
}

// 命令面板（Cmd+Shift+P）
const showCommandPalette = ref(false)
const openCommandPalette = () => {
  showCommandPalette.value = true
}

// 差异对比 / 实时预览
const showDiff = ref(false)
const showPreview = ref(false)
const openDiff = () => {
  if (!currentFilePath.value) {
    toast.info(t('app.diffNeedSaved'))
    return
  }
  showDiff.value = true
}
const togglePreview = () => {
  showPreview.value = !showPreview.value
}

// ===== Git 源代码管理 =====
const showGit = ref(false)
const openGit = () => {
  if (!rootDir.value) {
    toast.info(t('app.openFolderFirst'))
    return
  }
  showGit.value = true
}

// 文件树徽标用：绝对路径 → 状态字母（M/A/D/U）
const gitStatus = ref<Record<string, string>>({})
const gitRepo = ref(false)
// 计算单个根的 Git 状态（路径用绝对路径作 key，便于多根合并到同一张表）
const gitStatusFor = async (root: string): Promise<{ isRepo: boolean, map: Record<string, string> }> => {
  try {
    const s = await invoke<{ is_repo: boolean, files: { path: string, index: string, worktree: string }[] }>(
        'git_status', {root}
    )
    const map: Record<string, string> = {}
    if (s.is_repo) {
      for (const f of s.files) {
        const code = f.index === '?'
            ? 'U'
            : (f.worktree.trim() || f.index.trim() || 'M')
        map[`${root}/${f.path}`] = code
      }
    }
    return {isRepo: s.is_repo, map}
  }
  catch {
    return {isRepo: false, map: {}}
  }
}
const refreshGitStatus = async () => {
  if (!rootDir.value) {
    gitStatus.value = {}
    gitRepo.value = false
    return
  }
  const primary = await gitStatusFor(rootDir.value)
  gitRepo.value = primary.isRepo
  const map: Record<string, string> = {...primary.map}
  // 额外挂载的根各自可为独立仓库，合并它们的状态徽标
  if (extraRoots.value.length) {
    const extra = await Promise.all(extraRoots.value.map(er => gitStatusFor(er)))
    for (const e of extra) Object.assign(map, e.map)
  }
  gitStatus.value = map
  // HEAD 可能因提交/切换分支变化，刷新编辑器行内差异基线
  fetchBaseline()
}

// ===== 编辑器行内差异标记（vs HEAD）=====
// 当前文件在 HEAD 中的内容；null 表示无基线（新文件/非 git/未跟踪），不显示标记
const gitBaseline = ref<string | null>(null)

const fetchBaseline = async () => {
  if (!rootDir.value || !currentFilePath.value || !currentFilePath.value.startsWith(rootDir.value)) {
    gitBaseline.value = null
    applyDiffMarkers()
    return
  }
  const rel = currentFilePath.value.slice(rootDir.value.length + 1)
  try {
    const head = await invoke<{ exists: boolean, content: string }>('git_file_head', {
      root: rootDir.value,
      relPath: rel
    })
    gitBaseline.value = head.exists ? head.content : null
  }
  catch {
    gitBaseline.value = null
  }
  applyDiffMarkers()
}

// 计算并派发标记到编辑器
const applyDiffMarkers = () => {
  const view = editorView.value
  if (!view) {
    return
  }
  const markers = gitBaseline.value === null
      ? {changed: new Map(), deleted: new Set<number>()}
      : computeDiffMarkers(gitBaseline.value, code.value)
  view.dispatch({effects: setDiffMarkers.of(markers)})
}
const applyDiffMarkersDebounced = debounce(applyDiffMarkers, 250)

// 切换文件取新基线；编辑时重算；编辑器重挂时重新派发
watch(currentFilePath, () => fetchBaseline())
watch(code, () => applyDiffMarkersDebounced())
watch(editorView, () => applyDiffMarkers())

// ===== 断点（B1-P2）：把当前文件的断点 + 执行行派发到编辑器 =====
const debug = useDebug()
const sqlTxn = useSqlTxn()

// 事务开启/提交/回滚的反馈写入结果面板
const onTxnNotice = (text: string) => {
  if (layoutMode.value === 'editor') {
    showConsole.value = true
  }
  output.value = JSON.stringify({result_sets: [], messages: [text], error: null})
  isSuccess.value = true
}
const applyBreakpoints = () => {
  const view = editorView.value
  if (!view) {
    return
  }
  const path = currentFilePath.value
  const lines = debug.fileBreakpoints(path)
  const exec = debug.stopped.value && debug.stopped.value.path === path ? debug.stopped.value.line : null
  view.dispatch({effects: setBreakpointData.of({lines, exec})})
}
watch(() => debug.bpVersion.value, () => {
  applyBreakpoints()
  // 会话进行中：实时下发当前文件断点
  debug.syncBreakpoints(currentFilePath.value)
})
watch(() => debug.stopped.value, () => applyBreakpoints())
watch(editorView, () => applyBreakpoints())
watch(currentFilePath, () => applyBreakpoints())

// 停驻 / 选择调用栈帧时打开对应文件并跳转
watch(() => debug.reveal.value, async (loc) => {
  if (!loc) {
    return
  }
  if (loc.path !== currentFilePath.value) {
    await smartOpen(loc.path)
    await nextTick()
  }
  gotoLine(loc.line)
})

// 打开文件夹、保存文件、挂载/移除额外根后刷新文件树 Git 徽标与差异基线
watch([rootDir, extraRoots], () => refreshGitStatus(), {immediate: true, deep: true})
watch(savedContent, () => refreshGitStatus())

const closeViewer = () => {
  showViewer.value = false
  viewerFile.value = null
}

const {
  showAbout,
  showSettings,
  showUpdate,
  closeAbout,
  closeSettings,
  closeUpdate
} = useAppState()

// 编辑器配置管理
const {
  editorConfig,
  loadConfig: loadEditorConfig
} = useEditorConfig()

// 强制刷新 CodeEditor 组件的 key
const editorConfigKey = ref(0)
const consoleType = ref('console')

// 语言变化（打开文件/切换标签/下拉切换）时同步输出类型，否则 JSON/Markdown 等视图不会激活
watch(currentLanguage, () => {
  consoleType.value = getCurrentConsoleType()
})

// ===== 布局管理 =====
// 当前布局模式：horizontal(左右) / vertical(上下) / editor(仅编辑器)
const layoutMode = computed<LayoutMode>(() => editorConfig.value?.layout || 'horizontal')

// 控制台是否展开（仅编辑器模式下运行后才展开）
const showConsole = ref(true)

// 实际分割方向：仅编辑器模式按上次保存的方向弹出
const effectiveDirection = computed<SplitDirection>(() => {
  if (layoutMode.value === 'editor') {
    return editorConfig.value?.last_direction || 'horizontal'
  }
  return layoutMode.value
})

// 不同方向使用不同的最小尺寸
const minPrimary = computed(() => effectiveDirection.value === 'vertical' ? 200 : 400)
const minSecondary = computed(() => effectiveDirection.value === 'vertical' ? 150 : 300)

// 布局模式变化时同步控制台展开状态
watch(layoutMode, (mode) => {
  showConsole.value = mode !== 'editor'
}, {immediate: true})

const handleLayoutChange = (mode: LayoutMode) => {
  if (!editorConfig.value) {
    return
  }
  editorConfig.value.layout = mode
  // 记录最近使用的分割方向，供仅编辑器模式弹出时复用
  if (mode === 'horizontal' || mode === 'vertical') {
    editorConfig.value.last_direction = mode
  }
}

// 运行输入：参数 + stdin + 环境变量
const showRunInput = ref(false)
const runArgs = ref('')
const runStdin = ref('')
const runEnv = ref('')

// 监听模式：保存后自动运行
const watchMode = ref(kvGet('watch-mode') === 'true')
watch(watchMode, (v) => kvSet('watch-mode', String(v)))

// 保存包装：保存后若开启监听模式则自动运行
// 保存前按 .editorconfig 清理：去除行尾空白 / 补末尾换行（无 .editorconfig 时为空操作）
const applyEditorconfigOnSave = async () => {
  const view = editorView.value
  if (!view || !currentFilePath.value) {
    return
  }
  let r: { trim_trailing_whitespace?: boolean; insert_final_newline?: boolean } | null = null
  try {
    r = await invoke('resolve_editorconfig', {filePath: currentFilePath.value})
  }
  catch {
    return
  }
  const trim = r?.trim_trailing_whitespace === true
  const finalNl = r?.insert_final_newline === true
  if (!trim && !finalNl) {
    return
  }
  const text = view.state.doc.toString()
  let next = text
  if (trim) {
    next = next.replace(/[ \t]+(\r?\n)/g, '$1').replace(/[ \t]+$/, '')
  }
  if (finalNl && next.length > 0 && !next.endsWith('\n')) {
    next += '\n'
  }
  if (next === text) {
    return
  }
  const head = view.state.selection.main.head
  view.dispatch({
    changes: {from: 0, to: view.state.doc.length, insert: next},
    selection: {anchor: Math.min(head, next.length)}
  })
  await nextTick()
}

const handleSave = async () => {
  // 保存前格式化：仅当开启、编辑器就绪且当前语言支持 LSP；失败/无能力则静默跳过
  if (formatOnSave.value && editorView.value && lspSupportsLanguage(currentLanguage.value)) {
    try {
      await formatDocumentAsync(editorView.value)
      await nextTick()
    }
    catch { /* 格式化失败不阻断保存 */ }
  }
  await applyEditorconfigOnSave()
  await saveFile()
  if (watchMode.value && currentFilePath.value && !isDirty.value) {
    handleRunCode()
  }
}

// ===== 按文件记忆运行配置（args/stdin/env）=====
const RUN_CONFIGS_KEY = 'run-configs'
type RunConfig = { args: string, stdin: string, env: string }
const loadRunConfigs = (): Record<string, RunConfig> =>
    kvGetJSON<Record<string, RunConfig>>(RUN_CONFIGS_KEY, {})
// 把当前输入写入指定文件的配置（全空则删除该条）
const saveRunConfig = (path: string) => {
  const map = loadRunConfigs()
  if (!runArgs.value && !runStdin.value && !runEnv.value) {
    delete map[path]
  }
  else {
    map[path] = {args: runArgs.value, stdin: runStdin.value, env: runEnv.value}
  }
  kvSetJSON(RUN_CONFIGS_KEY, map)
}
// 载入指定文件的配置（无则清空）
const loadRunConfig = (path: string | null) => {
  const cfg = path ? loadRunConfigs()[path] : null
  runArgs.value = cfg?.args || ''
  runStdin.value = cfg?.stdin || ''
  runEnv.value = cfg?.env || ''
}

// 切换文件时：保存旧文件输入、载入新文件输入
watch(currentFilePath, (np, op) => {
  if (op) {
    saveRunConfig(op)
  }
  loadRunConfig(np)
})

// 编辑输入时防抖保存到当前文件
const persistRunConfig = debounce(() => {
  if (currentFilePath.value) {
    saveRunConfig(currentFilePath.value)
  }
}, 400)
watch([runArgs, runStdin, runEnv], () => persistRunConfig())

// 解析环境变量文本（KEY=值，按换行或分号分隔）
const parseEnv = (text: string): Record<string, string> => {
  const env: Record<string, string> = {}
  for (const part of text.split(/[\n;]/)) {
    const seg = part.trim()
    if (!seg) continue
    const eq = seg.indexOf('=')
    if (eq > 0) {
      env[seg.slice(0, eq).trim()] = seg.slice(eq + 1).trim()
    }
  }
  return env
}

const buildRunBase = () => {
  const env = parseEnv(runEnv.value)
  return {
    language: currentLanguage.value,
    envInstalled: envInfo.value.installed,
    envLanguage: envInfo.value.language,
    args: runArgs.value.trim() ? runArgs.value.trim().split(/\s+/) : undefined,
    stdin: runStdin.value || undefined,
    env: Object.keys(env).length ? env : undefined
  }
}

// 运行未保存文件的询问弹窗
const showRunPrompt = ref(false)

// 包装运行：仅编辑器模式下点击运行时自动展开控制台；关联文件则按策略就地运行
// 运行选中片段：以选中文本作为临时代码运行（不就地、不关联文件）
// SQL 走专用执行（结构化结果 + 错误 + 数据源：内存/SQLite/MySQL）
const {resolveActiveSource} = useDbConnections()

// 结果分页：超大结果集按页拉取，避免一次性取全量
const SQL_PAGE_SIZE = 500
const sqlPage = reactive<{ active: boolean; sql: string; source: any; offset: number; hasMore: boolean }>({
  active: false, sql: '', source: null, offset: 0, hasMore: false
})
const sqlPaging = computed(() => ({active: sqlPage.active, offset: sqlPage.offset, pageSize: SQL_PAGE_SIZE, hasMore: sqlPage.hasMore}))
// 单条 SELECT/WITH 才可分页（去掉尾分号后无其它分号，且以 select/with 开头）
const isPageableSql = (sql: string): boolean => {
  const s = sql.trim().replace(/;\s*$/, '')
  return !s.includes(';') && /^(select|with)\b/i.test(s)
}

const loadSqlPage = async (offset: number, record: boolean) => {
  if (layoutMode.value === 'editor') {
    showConsole.value = true
  }
  isRunning.value = true
  try {
    const res = await invoke<any>('run_sql_paged', {
      sql: sqlPage.sql, source: sqlPage.source, limit: SQL_PAGE_SIZE, offset, record
    })
    output.value = JSON.stringify(res)
    isSuccess.value = !res.error
    lastExecutionTime.value = res.elapsed_ms || 0
    sqlPage.offset = offset
    sqlPage.hasMore = ((res.result_sets || [])[0]?.rows || []).length === SQL_PAGE_SIZE
    if (res.error) {
      toast.error(t('app.sqlFailed'))
    }
  }
  catch (error) {
    output.value = JSON.stringify({result_sets: [], messages: [], error: String(error)})
    toast.error(t('app.sqlFailedColon') + error)
  }
  finally {
    isRunning.value = false
  }
}
const sqlPrevPage = () => sqlPage.offset > 0 && loadSqlPage(Math.max(0, sqlPage.offset - SQL_PAGE_SIZE), false)
const sqlNextPage = () => sqlPage.hasMore && loadSqlPage(sqlPage.offset + SQL_PAGE_SIZE, false)

const runSql = async (sqlOverride?: string) => {
  const sql = sqlOverride ?? code.value
  if (!sql.trim()) {
    toast.info(t('app.noSql'))
    return
  }
  // 事务进行中：整段在持有连接上执行（不走分页，否则分页用的是另一条连接看不到未提交数据）
  if (sqlTxn.active.value) {
    output.value = ''
    isSuccess.value = false
    if (layoutMode.value === 'editor') {
      showConsole.value = true
    }
    isRunning.value = true
    try {
      const res = await sqlTxn.exec(sql)
      output.value = JSON.stringify(res)
      isSuccess.value = !res.error
      lastExecutionTime.value = res.elapsed_ms || 0
      if (res.error) {
        toast.error(t('app.sqlFailed'))
      }
    }
    catch (error) {
      output.value = JSON.stringify({result_sets: [], messages: [], error: String(error)})
      toast.error(t('app.sqlFailedColon') + error)
    }
    finally {
      isRunning.value = false
    }
    return
  }
  const source = resolveActiveSource()
  output.value = ''
  isSuccess.value = false
  // 可分页查询：走分页拉取（首页记入历史）
  if (isPageableSql(sql)) {
    sqlPage.active = true
    sqlPage.sql = sql
    sqlPage.source = source
    sqlPage.offset = 0
    await loadSqlPage(0, true)
    return
  }
  sqlPage.active = false
  if (layoutMode.value === 'editor') {
    showConsole.value = true
  }
  isRunning.value = true
  try {
    const res = await invoke<any>('run_sql', {sql, source})
    output.value = JSON.stringify(res)
    isSuccess.value = !res.error
    lastExecutionTime.value = res.elapsed_ms || 0
    if (res.error) {
      toast.error(t('app.sqlFailed'))
    }
  }
  catch (error) {
    output.value = JSON.stringify({result_sets: [], messages: [], error: String(error)})
    toast.error(t('app.sqlFailedColon') + error)
  }
  finally {
    isRunning.value = false
  }
}

// 表结构浏览器：把预览 SQL 写入编辑器并运行（编辑器与运行保持一致）
const previewTable = (sql: string) => {
  const view = editorView.value
  if (view) {
    view.dispatch({changes: {from: 0, to: view.state.doc.length, insert: sql}})
  }
  runSql(sql)
}

// 表结构浏览器：在光标处插入表名/列名
const insertAtCursor = (text: string) => {
  const view = editorView.value
  if (!view) {
    return
  }
  const {from, to} = view.state.selection.main
  view.dispatch({
    changes: {from, to, insert: text},
    selection: {anchor: from + text.length}
  })
  view.focus()
}

const runSelection = () => {
  const view = editorView.value
  if (!view) {
    return
  }
  const {from, to} = view.state.selection.main
  if (from === to) {
    toast.info(t('app.selectCodeFirst'))
    return
  }
  const selected = view.state.sliceDoc(from, to)
  if (currentLanguage.value === 'sql') {
    runSql(selected)
    return
  }
  if (layoutMode.value === 'editor') {
    showConsole.value = true
  }
  runCode({...buildRunBase(), codeOverride: selected})
}

const handleRunCode = async () => {
  if (currentLanguage.value === 'sql') {
    runSql()
    return
  }
  if (layoutMode.value === 'editor') {
    showConsole.value = true
  }

  // 草稿（无关联文件）：临时目录运行
  if (!currentFilePath.value) {
    runCode(buildRunBase())
    return
  }
  // 无改动：直接就地运行
  if (!isDirty.value) {
    runCode({...buildRunBase(), filePath: currentFilePath.value})
    return
  }

  // 有未保存改动：按设置的策略处理
  const strategy = editorConfig.value?.run_save_strategy || 'auto-save'
  if (strategy === 'temp-copy') {
    runCode(buildRunBase()) // 跑当前未保存内容的临时副本
  }
  else if (strategy === 'ask') {
    showRunPrompt.value = true
  }
  else {
    await applyEditorconfigOnSave()
    await saveFile()
    runCode({...buildRunBase(), filePath: currentFilePath.value})
  }
}

const promptSaveAndRun = async () => {
  showRunPrompt.value = false
  await applyEditorconfigOnSave()
  await saveFile()
  runCode({...buildRunBase(), filePath: currentFilePath.value})
}

const promptRunCopy = () => {
  showRunPrompt.value = false
  runCode(buildRunBase())
}

// 设置关闭后刷新缓存的编辑器配置与快捷键绑定，使其即时生效
const onSettingsClose = async () => {
  closeSettings()
  await loadEditorConfig()
  reloadShortcuts()
}

const handleSettingsChanged = async (config: any) => {
  console.log('主组件接收到设置变更:', config)
  setTimeout(() => {
    editorConfigKey.value++
  }, 50)

  await refreshLanguageList()
  await buildLanguageRegistry()
}

const loadExample = (content: string) => {
  code.value = content || ''
  // 示例内容不对应任何本地文件，解除文件关联
  resetFile()
}

const restoreHistoryItem = (item: ExecutionResult) => {
  applyLanguage(item.language)
  code.value = item.code || ''
  resetFile()
  clearOutput()
  toast.success(t('app.historyRestored'))
}

// 从执行历史一键重跑：恢复语言与代码后直接运行
const rerunHistoryItem = async (item: ExecutionResult) => {
  applyLanguage(item.language)
  code.value = item.code || ''
  resetFile()
  // 确保环境信息已更新为该语言再运行
  await refreshEnvInfo()
  handleRunCode()
}

// 监听编辑器配置变化
watch(editorConfig, (newConfig) => {
  if (newConfig) {
    console.log('编辑器配置更新，刷新编辑器组件')
    setTimeout(() => {
      editorConfigKey.value++
    }, 50)
  }
}, {deep: true})

watch(currentLanguage, () => {
  consoleType.value = getCurrentConsoleType()
})

const {initializeEventListeners, cleanupEventListeners} = useEventManager({
  showAbout,
  showSettings,
  showUpdate,
  output,
  isRunning,
  isSuccess,
  lastExecutionTime,
  toast,
  handleRealtimeOutput,
  handleExecutionComplete,
  handleExecutionStopped,
  handleExecutionTimeout,
  handleExecutionError
})

// 禁用右键菜单
window.addEventListener('contextmenu', (e) => e.preventDefault(), false)

// 是否有弹窗/覆盖层打开（打开时不响应全局快捷键）
const isOverlayOpen = () =>
    showSettings.value || showAbout.value || showUpdate.value
    || showHistory.value || showViewer.value || showRunPrompt.value
    || showQuickOpen.value || showGenerate.value || showSearch.value
    || showCommandPalette.value || showDiff.value || showGoToLine.value || showOutline.value || showSnippets.value
    || applyPreview.value != null

// 全局快捷键（绑定可在设置中自定义）
const {matchAction: matchShortcut, reload: reloadShortcuts, getBinding, formatCombo} = useShortcuts()

const shortcutDispatch: Record<string, () => void> = {
  run: () => handleRunCode(),
  runSelection: () => runSelection(),
  quickOpen: () => openQuickOpen(),
  commandPalette: () => openCommandPalette(),
  gotoLine: () => openGoToLine(),
  outline: () => openOutline(),
  searchInFiles: () => openSearch(),
  generate: () => openGenerate(),
  save: () => handleSave(),
  saveAs: () => saveFileAs(),
  open: () => handleOpenFileClick(),
  newTab: () => handleNewTab(),
  closeTab: () => handleCloseTab(activeTabId.value),
  reopenClosed: () => handleReopenClosed(),
  toggleSidebar: () => toggleSidebar(),
  toggleTerminal: () => toggleTerminal(),
  toggleWordWrap: () => toggleWordWrap()
}

// 切换自动换行（即时生效并随编辑器配置持久化）
const toggleWordWrap = () => {
  if (!editorConfig.value) {
    return
  }
  editorConfig.value.word_wrap = !editorConfig.value.word_wrap
  toast.info(editorConfig.value.word_wrap ? t('app.wordWrapOn') : t('app.wordWrapOff'))
}

// 切换并持久化外观主题
const applyTheme = async (t: AppTheme) => {
  setAppTheme(t)
  try {
    const config = await invoke<any>('get_app_config')
    config.theme = t
    await invoke('update_app_config', {config})
  }
  catch (error) {
    console.error('保存主题失败:', error)
  }
}

// 命令面板命令列表（含快捷键提示）
const hintOf = (id: string) => formatCombo(getBinding(id))
const paletteCommands = computed<PaletteCommand[]>(() => [
  {id: 'run', label: t('command.run'), icon: Play, hint: hintOf('run'), run: () => handleRunCode()},
  {id: 'runSelection', label: t('command.runSelection'), icon: Play, hint: hintOf('runSelection'), run: () => runSelection()},
  {id: 'watchMode', label: watchMode.value ? t('command.watchModeOff') : t('command.watchModeOn'), icon: Eye, run: () => { watchMode.value = !watchMode.value }},
  {id: 'aiCompletion', label: aiCompletion.value ? t('command.aiCompletionOff') : t('command.aiCompletionOn'), icon: Sparkles, run: () => toggleAiCompletion()},
  {id: 'formatOnSave', label: formatOnSave.value ? t('command.formatOnSaveOff') : t('command.formatOnSaveOn'), icon: Save, run: () => toggleFormatOnSave()},
  {id: 'open', label: t('command.open'), icon: FolderOpen, hint: hintOf('open'), run: () => handleOpenFileClick()},
  {id: 'openFolder', label: t('command.openFolder'), icon: FolderOpen, run: () => openFolder()},
  {id: 'save', label: t('command.save'), icon: Save, hint: hintOf('save'), run: () => handleSave()},
  {id: 'saveAs', label: t('command.saveAs'), icon: Save, hint: hintOf('saveAs'), run: () => saveFileAs()},
  {id: 'newTab', label: t('command.newTab'), icon: Plus, hint: hintOf('newTab'), run: () => handleNewTab()},
  {id: 'closeTab', label: t('command.closeTab'), icon: X, hint: hintOf('closeTab'), run: () => handleCloseTab(activeTabId.value)},
  {id: 'reopenClosed', label: t('command.reopenClosed'), icon: Plus, hint: hintOf('reopenClosed'), run: () => handleReopenClosed()},
  {id: 'quickOpen', label: t('command.quickOpen'), icon: Search, hint: hintOf('quickOpen'), run: () => openQuickOpen()},
  {id: 'gotoLine', label: t('command.gotoLine'), icon: CornerDownRight, hint: hintOf('gotoLine'), run: () => openGoToLine()},
  {id: 'outline', label: t('command.outline'), icon: ListTree, hint: hintOf('outline'), run: () => openOutline()},
  {id: 'snippets', label: t('command.snippets'), icon: Code2, run: () => { showSnippets.value = true }},
  {id: 'terminal', label: t('command.terminal'), icon: TerminalIcon, hint: hintOf('toggleTerminal'), run: () => toggleTerminal()},
  {id: 'searchInFiles', label: t('command.searchInFiles'), icon: Search, hint: hintOf('searchInFiles'), run: () => openSearch()},
  {id: 'generate', label: t('command.generate'), icon: Sparkles, hint: hintOf('generate'), run: () => openGenerate()},
  {id: 'showAi', label: t('command.showAi'), icon: Sparkles, run: () => handleShowAi()},
  {id: 'explainCode', label: t('command.explainCode'), icon: Sparkles, run: () => explainCode()},
  {id: 'generateTests', label: t('command.generateTests'), icon: Sparkles, run: () => generateTests()},
  {id: 'formatWithAi', label: t('command.formatWithAi'), icon: Sparkles, run: () => formatWithAi()},
  {id: 'history', label: t('command.history'), icon: History, run: () => { showHistory.value = true }},
  {id: 'diff', label: t('command.diff'), icon: GitCompare, run: () => openDiff()},
  {id: 'preview', label: t('command.preview'), icon: Eye, run: () => togglePreview()},
  {id: 'git', label: t('command.git'), icon: GitBranch, run: () => openGit()},
  {id: 'tasks', label: t('command.tasks'), icon: ListChecks, run: () => openTasks()},
  {id: 'gitignore', label: t('command.gitignore'), icon: GitBranch, run: () => openGitignore()},
  {id: 'runTests', label: t('command.runTests'), icon: ListChecks, run: () => runTests()},
  {id: 'startDebug', label: t('command.startDebug'), icon: Play, run: () => startDebug()},
  {id: 'sendToTerminal', label: t('command.sendToTerminal'), icon: TerminalIcon, run: () => sendToTerminal()},
  {id: 'revealInTree', label: t('command.revealInTree'), icon: FolderOpen, run: () => revealInTree()},
  {id: 'copyPermalink', label: t('command.copyPermalink'), icon: GitBranch, run: () => copyPermalink()},
  {id: 'openPermalink', label: t('command.openPermalink'), icon: GitBranch, run: () => openPermalink()},
  {id: 'toggleAutoReveal', label: t('command.toggleAutoReveal'), icon: FolderOpen, run: () => toggleAutoReveal()},
  {id: 'toggleSidebar', label: t('command.toggleSidebar'), icon: PanelLeft, hint: hintOf('toggleSidebar'), run: () => toggleSidebar()},
  {id: 'toggleWordWrap', label: t('command.toggleWordWrap'), icon: WrapText, hint: hintOf('toggleWordWrap'), run: () => toggleWordWrap()},
  {id: 'layoutHorizontal', label: t('command.layoutHorizontal'), group: t('command.groupLayout'), icon: PanelRight, run: () => handleLayoutChange('horizontal')},
  {id: 'layoutVertical', label: t('command.layoutVertical'), group: t('command.groupLayout'), icon: PanelBottom, run: () => handleLayoutChange('vertical')},
  {id: 'layoutEditor', label: t('command.layoutEditor'), group: t('command.groupLayout'), icon: Maximize2, run: () => handleLayoutChange('editor')},
  {id: 'themeSystem', label: t('command.themeSystem'), group: t('command.groupTheme'), icon: Monitor, run: () => applyTheme('system')},
  {id: 'themeLight', label: t('command.themeLight'), group: t('command.groupTheme'), icon: Sun, run: () => applyTheme('light')},
  {id: 'themeDark', label: t('command.themeDark'), group: t('command.groupTheme'), icon: Moon, run: () => applyTheme('dark')},
  {id: 'settings', label: t('command.settings'), icon: SettingsIcon, run: () => { showSettings.value = true }}
])

const onGlobalKeydown = (e: KeyboardEvent) => {
  if (isOverlayOpen()) {
    return
  }
  const action = matchShortcut(e)
  if (action && shortcutDispatch[action]) {
    // 捕获阶段拦截：阻止事件到达编辑器（避免 Cmd+Enter 等被插入换行）
    e.preventDefault()
    e.stopPropagation()
    shortcutDispatch[action]()
  }
}

const {init: initTheme, setTheme: setAppTheme} = useTheme()

onMounted(async () => {
  await initTheme()
  await initialize()
  await buildLanguageRegistry()
  // 以当前内容初始化首个标签页
  initFirstTab()
  await loadEditorConfig()
  await initializeEventListeners()
  consoleType.value = getCurrentConsoleType()
  // 从数据库载入代码片段
  initSnippets()

  // 恢复上次打开的文件夹
  const lastRoot = kvGet(LAST_ROOT_KEY)
  if (lastRoot) {
    rootDir.value = lastRoot
  }

  // 恢复上次打开的文件标签
  await restoreSession()

  window.addEventListener('keydown', onGlobalKeydown, true)
  window.addEventListener('lsp:open-location', onLspOpenLocation)
  window.addEventListener('lsp:code-actions', onLspCodeActions)
  window.addEventListener('contextmenu', onEditorContext)

  // 触发 app-ready 事件，通知主进程
  window.dispatchEvent(new CustomEvent('app-ready'))
})

onUnmounted(() => {
  cleanupEventListeners()
  window.removeEventListener('keydown', onGlobalKeydown, true)
  window.removeEventListener('lsp:open-location', onLspOpenLocation)
  window.removeEventListener('lsp:code-actions', onLspCodeActions)
  window.removeEventListener('contextmenu', onEditorContext)
})
</script>
