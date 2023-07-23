export function startCanvas(renderCanvas) {
  const scene = new THREE.Scene();
  const camera = new THREE.PerspectiveCamera(
    75,
    renderCanvas.clientWidth / renderCanvas.clientHeight,
    0.1,
    1000
  );
  const renderer = new THREE.WebGLRenderer({ canvas: renderCanvas });
  renderer.setSize(renderCanvas.clientWidt, renderCanvas.clientHeight);
  // move camera back
  camera.position.z = 5;
  // render scene
  renderer.render(scene, camera);

  const animate = () => {
    requestAnimationFrame(animate);
    renderer.render(scene, camera);
  };
  animate();

  // add resize watch of renderCanvas
  const resizeObserver = new ResizeObserver((entries) => {
    for (let entry of entries) {
      const { width, height } = entry.contentRect;
      renderer.setSize(width, height);
      camera.aspect = width / height;
      camera.updateProjectionMatrix();
    }
  });
  resizeObserver.observe(renderCanvas);
  return scene;
}
