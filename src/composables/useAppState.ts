import { ref } from 'vue'

export function useAppState()
{
    const showAbout = ref(false)
    const showSettings = ref(false)
    const activeTab = ref('output')

    const closeAbout = () => {
        showAbout.value = false
    }

    const closeSettings = () => {
        showSettings.value = false
    }

    const setActiveTab = (tab: string) => {
        activeTab.value = tab
    }

    return {
        showAbout,
        showSettings,
        activeTab,
        closeAbout,
        closeSettings,
        setActiveTab
    }
}
