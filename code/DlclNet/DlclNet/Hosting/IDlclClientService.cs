using DlclNet.Models;
using DlclRpc;

namespace DlclNet.Hosting;

/// <summary>
/// DLCL Client Service for use in Hosting (e.g. with ASP.Net or other hosted systems)
/// </summary>
public interface IDlclClientService
{
    /// <summary>
    ///<inheritdoc cref="DlclClient.GetAnimatedLayersAsync"/>
    /// </summary>
    /// <param name="cancellationToken"></param>
    /// <returns></returns>
    Task<uint[]> GetAnimatedLayersAsync(CancellationToken cancellationToken = default);

    /// <summary>
    /// <inheritdoc cref="DlclClient.PushAnimationsAsync"/>
    /// </summary>
    /// <param name="animations"></param>
    /// <param name="cancellationToken"></param>
    /// <returns></returns>
    Task<StatusResponse> PushAnimationsAsync(IEnumerable<Animation> animations, CancellationToken cancellationToken = default);

    /// <summary>
    /// <inheritdoc cref="DlclClient.DrawOnLayerAsync"/>
    /// </summary>
    /// <param name="pixels"></param>
    /// <param name="layerId"></param>
    /// <param name="cancellationToken"></param>
    /// <returns></returns>
    Task<StatusResponse> DrawOnLayerAsync(IEnumerable<Pixel> pixels, uint layerId, CancellationToken cancellationToken = default);

    /// <summary>
    /// <inheritdoc cref="DlclClient.DrawFullLayerAsync"/>
    /// </summary>
    /// <param name="frame"></param>
    /// <param name="layerId"></param>
    /// <param name="cancellationToken"></param>
    /// <returns></returns>
    Task<StatusResponse> DrawFullLayerAsync(SingleFrame frame, uint layerId, CancellationToken cancellationToken = default);

    /// <summary>
    /// <inheritdoc cref="DlclClient.DrawDirectAsync"/>
    /// </summary>
    /// <param name="pixels"></param>
    /// <param name="cancellationToken"></param>
    /// <returns></returns>
    Task<StatusResponse> DrawDirectAsync(IEnumerable<Pixel> pixels, CancellationToken cancellationToken = default);

    /// <summary>
    /// <inheritdoc cref="DlclClient.GetAnimationQueueAsync"/>
    /// </summary>
    /// <param name="cancellationToken"></param>
    /// <returns></returns>
    Task<List<Animation>> GetAnimationQueueAsync(CancellationToken cancellationToken = default);
}