import {ref} from 'vue'

export function useAppState()
{
    const showAbout = ref(false)
    const showSettings = ref(false)
    const showUpdate = ref(false)

    const closeAbout = () => {
        showAbout.value = false
    }

    const closeSettings = () => {
        showSettings.value = false
    }

    const closeUpdate = () => {
        showUpdate.value = false
    }

    return {
        showAbout,
        showSettings,
        showUpdate,
        closeAbout,
        closeSettings,
        closeUpdate
    }
}
