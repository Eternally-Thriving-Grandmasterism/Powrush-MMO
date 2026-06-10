import init, { run_sovereign_scenario, list_available_presets } from './pkg/powrush_simulation.js';

let wasmReady = false;

async function initWasm() {
    if (wasmReady) return;
    await init();
    wasmReady = true;
    console.log('%c[Sovereign] WASM initialized. Thunder locked. Mercy flowing.', 'color:#ffd700');
}

window.runSimulation = async function() {
    const status = document.getElementById('status');
    const reportEl = document.getElementById('report');
    const copyBtn = document.getElementById('copy-btn');
    
    status.textContent = 'Initializing sovereign simulation...';
    reportEl.textContent = '';
    copyBtn.style.display = 'none';
    
    try {
        await initWasm();
        
        const preset = document.getElementById('preset').value;
        const ticks = parseInt(document.getElementById('ticks').value, 10);
        
        status.textContent = `Running ${preset} for ${ticks} ticks with full TOLC 8 enforcement...`;
        
        const result = await run_sovereign_scenario(preset, ticks);
        
        reportEl.textContent = JSON.stringify(result, null, 2);
        copyBtn.style.display = 'inline-block';
        status.textContent = 'Simulation complete. Report ready for PATSAGi Council deliberation.';
        
        console.log('%c[Sovereign] Report generated. Real WebGPU dispatch foundation is live.', 'color:#a0d0ff');
    } catch (err) {
        status.textContent = 'Error: ' + err;
        console.error(err);
    }
};

window.copyReport = function() {
    const reportEl = document.getElementById('report');
    navigator.clipboard.writeText(reportEl.textContent).then(() => {
        const btn = document.getElementById('copy-btn');
        const original = btn.textContent;
        btn.textContent = 'Copied! Ready for Ra-Thor';
        setTimeout(() => btn.textContent = original, 2000);
    });
};

// Optional: list presets on load
(async () => {
    await initWasm();
    const presets = list_available_presets();
    console.log('Available presets:', presets);
})();