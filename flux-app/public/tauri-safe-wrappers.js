// Safe wrappers for Tauri APIs that gracefully handle browser mode
// These prevent TypeErrors when __TAURI__ is undefined

window.__TAURI_SAFE__ = {
  // Safe invoke wrapper
  invoke: async function(cmd, args) {
    if (typeof window.__TAURI__ === 'undefined' ||
        typeof window.__TAURI__.core === 'undefined' ||
        typeof window.__TAURI__.core.invoke === 'undefined') {
      throw new Error('Tauri not available');
    }
    return await window.__TAURI__.core.invoke(cmd, args);
  },

  // Safe dialog.save wrapper
  dialogSave: async function(options) {
    if (typeof window.__TAURI__ === 'undefined' ||
        typeof window.__TAURI__.plugin === 'undefined' ||
        typeof window.__TAURI__.plugin.dialog === 'undefined' ||
        typeof window.__TAURI__.plugin.dialog.save === 'undefined') {
      throw new Error('Tauri not available');
    }
    return await window.__TAURI__.plugin.dialog.save(options);
  },

  // Safe dialog.open wrapper
  dialogOpen: async function(options) {
    if (typeof window.__TAURI__ === 'undefined' ||
        typeof window.__TAURI__.plugin === 'undefined' ||
        typeof window.__TAURI__.plugin.dialog === 'undefined' ||
        typeof window.__TAURI__.plugin.dialog.open === 'undefined') {
      throw new Error('Tauri not available');
    }
    return await window.__TAURI__.plugin.dialog.open(options);
  },

  // Safe event.listen wrapper
  listen: async function(event, handler) {
    if (typeof window.__TAURI__ === 'undefined' ||
        typeof window.__TAURI__.event === 'undefined' ||
        typeof window.__TAURI__.event.listen === 'undefined') {
      throw new Error('Tauri not available');
    }
    return await window.__TAURI__.event.listen(event, handler);
  }
};
